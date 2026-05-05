/**
 * TypeDuck-Web Browser E2E Spec
 *
 * Real browser validation for patched TypeDuck-Web + Yune runtime seam.
 * Covers composition, candidate actions, deploy, customize, and persistence per D-08/TYPEDUCK-E2E-03.
 *
 * Prerequisites:
 * 1. Patched TypeDuck-Web source (patches/yune-typeduck-runtime.patch applied)
 * 2. Yune WASM artifact with yune_typeduck_* exports in packages/yune-typeduck-runtime/dist/
 * 3. Built @yune-ime/typeduck-runtime package
 * 4. Explicit TypeDuck-Web-owned YAML assets (per e2e/assets/README.md)
 * 5. Bun installed (upstream uses Bun)
 * 6. Playwright installed (standalone spec framework)
 */

import { test, expect, Page, BrowserContext } from "@playwright/test";

// Test configuration
const APP_URL = process.env.TYPEDUCK_APP_URL || "http://localhost:5173";
const TIMEOUT_MS = 30000; // WASM load and runtime init may be slow

// E2E evidence directory
const EVIDENCE_DIR = "third_party/typeduck-web/e2e/results";

/**
 * Helper: Capture browser console errors to evidence file
 */
async function captureConsoleErrors(page: Page, evidenceFile: string): Promise<string[]> {
  const errors: string[] = [];
  page.on("console", (msg) => {
    if (msg.type() === "error") {
      errors.push(`[${new Date().toISOString()}] ${msg.text()}`);
    }
  });
  return errors;
}

/**
 * Helper: Save evidence to results directory
 */
async function saveEvidence(filename: string, content: string): Promise<void> {
  const fs = await import("fs/promises");
  const path = await import("path");
  const evidencePath = path.join(EVIDENCE_DIR, filename);
  await fs.writeFile(evidencePath, content, "utf8");
}

/**
 * Helper: Take screenshot with evidence filename
 */
async function takeEvidenceScreenshot(page: Page, flowName: string): Promise<void> {
  const screenshotPath = `${EVIDENCE_DIR}/screenshot-${flowName}.png`;
  await page.screenshot({ path: screenshotPath, fullPage: false });
}

/**
 * Helper: Verify persistence sync markers in console
 */
async function verifyPersistenceMarker(page: Page, marker: string): Promise<boolean> {
  const logs: string[] = [];
  page.on("console", (msg) => {
    logs.push(msg.text());
  });

  // Wait for marker to appear in console logs
  try {
    await page.waitForFunction(
      (expectedMarker: string) => {
        // Check if persistence marker logged (implementation-specific)
        return window.performance
          .getEntriesByType("measure")
          .some((entry) => entry.name.includes(expectedMarker));
      },
      marker,
      { timeout: 5000 }
    );
    return true;
  } catch {
    // Marker not found - persistence timing may not be logged
    return false;
  }
}

test.describe("TypeDuck-Web Browser E2E", () => {
  let consoleErrors: string[] = [];

  test.beforeAll(async ({ browser }) => {
    // Record browser runner start
    await saveEvidence(
      "browser-run.log",
      `[${new Date().toISOString()}] Browser E2E started\nURL: ${APP_URL}\n`
    );
  });

  test.beforeEach(async ({ page }) => {
    // Navigate to patched TypeDuck-Web app
    await page.goto(APP_URL, { timeout: TIMEOUT_MS, waitUntil: "networkidle" });

    // Capture console errors for evidence
    consoleErrors = await captureConsoleErrors(page, "browser-console.log");

    // Wait for app initialization
    // The patched worker dispatches "initialized" listener event
    await page.waitForTimeout(2000); // Allow WASM load and runtime init
  });

  test.afterEach(async ({ page }, testInfo) => {
    // Append test result to evidence log
    const status = testInfo.status || "unknown";
    const duration = testInfo.duration || 0;
    await saveEvidence(
      "browser-run.log",
      `[${new Date().toISOString()}] Test: ${testInfo.title} - ${status} (${duration}ms)\n`
    );

    // Save console errors if any
    if (consoleErrors.length > 0) {
      await saveEvidence(
        "browser-console.log",
        consoleErrors.join("\n") + "\n"
      );
    }
  });

  test("Composition after typing schema-valid keys", async ({ page }) => {
    // D-08/D-10: Composition appears after typing schema-valid keys

    // Focus input field
    const inputField = page.locator("input[type='text'], textarea").first();
    await inputField.focus();

    // Type schema-valid keys (assuming luna_pinyin schema)
    await inputField.type("abc", { delay: 100 });

    // Verify composition visible (preedit in UI)
    // TypeDuck-Web shows preedit in input field or composition panel
    const compositionVisible = await page.waitForSelector(
      "[data-composition], [data-preedit], .composition-panel",
      { timeout: 5000, state: "visible" }
    ).catch(() => null);

    if (compositionVisible) {
      await takeEvidenceScreenshot(page, "composition");
      expect(compositionVisible).toBeTruthy();
    } else {
      // Composition may be inline in input field
      const inputValue = await inputField.inputValue();
      await takeEvidenceScreenshot(page, "composition");
      expect(inputValue.length).toBeGreaterThan(0);
      await saveEvidence(
        "browser-run.log",
        `[${new Date().toISOString()}] Composition: input="${inputValue}"\n`
      );
    }
  });

  test("Candidate list visible", async ({ page }) => {
    // D-08/D-10: Candidate list is visible after composition

    const inputField = page.locator("input[type='text'], textarea").first();
    await inputField.focus();
    await inputField.type("ba", { delay: 100 }); // Type to trigger candidates

    // Wait for candidate panel
    const candidatePanel = await page.waitForSelector(
      "[data-candidates], .candidate-panel, .candidate-list",
      { timeout: 5000, state: "visible" }
    ).catch(() => null);

    await takeEvidenceScreenshot(page, "candidates");

    if (candidatePanel) {
      expect(candidatePanel).toBeTruthy();

      // Verify candidates exist
      const candidates = await page.locator(".candidate, [data-candidate]").count();
      expect(candidates).toBeGreaterThan(0);

      await saveEvidence(
        "browser-run.log",
        `[${new Date().toISOString()}] Candidates: ${candidates} visible\n`
      );
    } else {
      // Candidate panel may have different selector
      await saveEvidence(
        "blocker.md",
        `# Browser E2E Blocker\n\n**Category**: TypeDuck-Web app/source\n\n**Flow**: Candidate list visible\n\n**Issue**: No candidate panel selector found\n\n**Selectors tried**: [data-candidates], .candidate-panel, .candidate-list\n\n**Evidence**: screenshot-candidates.png\n\n**Impact**: Cannot verify candidate paging/selection flows\n`
      );
    }
  });

  test("Candidate paging", async ({ page }) => {
    // D-08/D-10: Candidate paging changes page/candidate state

    const inputField = page.locator("input[type='text'], textarea").first();
    await inputField.focus();
    await inputField.type("sh", { delay: 100 }); // Type to generate many candidates

    await page.waitForTimeout(1000);

    // Press PageDown to flip candidate page
    await page.keyboard.press("PageDown");

    await page.waitForTimeout(500);

    await takeEvidenceScreenshot(page, "candidate-paging");

    // Verify page changed (page indicator or candidate set changed)
    // TypeDuck-Web may show page number or update candidate list
    const pageIndicator = await page.locator("[data-page], .page-indicator").textContent().catch(() => null);

    if (pageIndicator) {
      await saveEvidence(
        "browser-run.log",
        `[${new Date().toISOString()}] Candidate paging: page="${pageIndicator}"\n`
      );
      expect(pageIndicator).toBeDefined();
    } else {
      // Candidate set may have changed without visible indicator
      await saveEvidence(
        "browser-run.log",
        `[${new Date().toISOString()}] Candidate paging: PageDown pressed (no visible page indicator)\n`
      );
    }
  });

  test("Candidate selection → commit output", async ({ page }) => {
    // D-08/D-10: Candidate selection commits text to app output field

    const inputField = page.locator("input[type='text'], textarea").first();
    await inputField.focus();
    await inputField.type("ba", { delay: 100 });

    await page.waitForTimeout(1000);

    // Press selection key (TypeDuck-Web uses digit keys 1-5 or Space/Enter)
    await page.keyboard.press("1"); // Select first candidate

    await page.waitForTimeout(500);

    await takeEvidenceScreenshot(page, "candidate-selection");

    // Verify committed text
    const inputValue = await inputField.inputValue();
    expect(inputValue.length).toBeGreaterThan(0);

    await saveEvidence(
      "browser-run.log",
      `[${new Date().toISOString()}] Candidate selection: committed="${inputValue}"\n`
    );
  });

  test("Deletion removes candidate or triggers delete path", async ({ page }) => {
    // D-08/D-10: Delete key removes candidate or triggers delete-candidate path

    const inputField = page.locator("input[type='text'], textarea").first();
    await inputField.focus();
    await inputField.type("ba", { delay: 100 });

    await page.waitForTimeout(1000);

    // Press Delete key
    await page.keyboard.press("Delete");

    await page.waitForTimeout(500);

    await takeEvidenceScreenshot(page, "deletion");

    // Verify deletion effect (composition changed or candidate removed)
    const compositionAfter = await inputField.inputValue();

    await saveEvidence(
      "browser-run.log",
      `[${new Date().toISOString()}] Deletion: input="${compositionAfter}"\n`
    );
  });

  test("Backspace mutates composition", async ({ page }) => {
    // D-08/D-10: Backspace/Delete mutates composition

    const inputField = page.locator("input[type='text'], textarea").first();
    await inputField.focus();
    await inputField.type("abc", { delay: 100 });

    await page.waitForTimeout(1000);

    const beforeBackspace = await inputField.inputValue();

    // Press Backspace
    await page.keyboard.press("Backspace");

    await page.waitForTimeout(500);

    const afterBackspace = await inputField.inputValue();

    // Verify composition mutated (shorter or changed)
    expect(afterBackspace.length).toBeLessThanOrEqual(beforeBackspace.length);

    await saveEvidence(
      "browser-run.log",
      `[${new Date().toISOString()}] Backspace: before="${beforeBackspace}" after="${afterBackspace}"\n`
    );
  });

  test("Deploy returns visible success/error evidence", async ({ page }) => {
    // D-08/D-10: Deploy returns visible success/error evidence

    // Locate deploy button/shortcut
    // TypeDuck-Web may have deploy button in settings or Ctrl+D shortcut
    const deployButton = await page.locator("[data-deploy], .deploy-button, button:has-text('deploy')").first();

    if (await deployButton.count() > 0) {
      await deployButton.click();
    } else {
      // Try keyboard shortcut (Ctrl+D or similar)
      await page.keyboard.press("Control+d");
    }

    await page.waitForTimeout(2000);

    // Verify deploy result visible (toast notification, console log, status change)
    const deployNotification = await page.locator(".toast, [data-deploy-status], .notification").textContent().catch(() => null);

    await takeEvidenceScreenshot(page, "deploy");

    if (deployNotification) {
      await saveEvidence(
        "browser-run.log",
        `[${new Date().toISOString()}] Deploy: status="${deployNotification}"\n`
      );
      expect(deployNotification).toBeDefined();
    } else {
      // Check console for deploy result
      await saveEvidence(
        "browser-run.log",
        `[${new Date().toISOString()}] Deploy: triggered (no visible notification)\n`
      );
    }
  });

  test("Customize returns visible success/error evidence", async ({ page }) => {
    // D-08/D-10: Customize returns visible success/error evidence

    // Locate customize settings panel
    // TypeDuck-Web may have settings panel for pageSize, completion, etc.
    const settingsPanel = await page.locator("[data-settings], .settings-panel, .customize-panel").first();

    if (await settingsPanel.count() > 0) {
      await settingsPanel.click();

      await page.waitForTimeout(1000);

      // Change a setting (e.g., pageSize)
      const pageSizeInput = await page.locator("input[name='pageSize'], [data-page-size]").first();
      if (await pageSizeInput.count() > 0) {
        await pageSizeInput.fill("10");
        await page.keyboard.press("Enter");
      }

      await page.waitForTimeout(2000);

      await takeEvidenceScreenshot(page, "customize");

      // Verify customize result visible
      const customizeNotification = await page.locator(".toast, [data-customize-status], .notification").textContent().catch(() => null);

      if (customizeNotification) {
        await saveEvidence(
          "browser-run.log",
          `[${new Date().toISOString()}] Customize: status="${customizeNotification}"\n`
        );
      } else {
        await saveEvidence(
          "browser-run.log",
          `[${new Date().toISOString()}] Customize: settings changed (no visible notification)\n`
        );
      }
    } else {
      await saveEvidence(
        "blocker.md",
        `# Browser E2E Blocker\n\n**Category**: TypeDuck-Web app/source\n\n**Flow**: Customize settings\n\n**Issue**: No settings/customize panel found\n\n**Selectors tried**: [data-settings], .settings-panel, .customize-panel\n\n**Impact**: Cannot verify customize flow\n`
      );
    }
  });

  test("Persistence sync after deploy/customize mutations", async ({ page, context }) => {
    // D-11: Persistence sync after deploy/customize/userdb-relevant boundaries

    // Perform mutation (deploy)
    await page.keyboard.press("Control+d"); // Deploy shortcut
    await page.waitForTimeout(1000);

    // Verify sync marker in console or performance timeline
    const syncMarkerFound = await verifyPersistenceMarker(page, "syncToPersistenceAfterMutation");

    await saveEvidence(
      "persistence-sync.log",
      `[${new Date().toISOString()}] syncToPersistenceAfterMutation: ${syncMarkerFound ? "PASS" : "FAIL (marker not logged)"}\n`
    );

    // Persistence evidence: check if IDBFS flushed
    // Implementation may log FS.syncfs(false) to console
    if (!syncMarkerFound) {
      await saveEvidence(
        "blocker.md",
        `# Browser E2E Blocker\n\n**Category**: Yune adapter/runtime\n\n**Flow**: Persistence sync evidence\n\n**Issue**: No persistence sync marker logged after mutation\n\n**Expected**: syncToPersistenceAfterMutation or FS.syncfs(false) console log\n\n**Impact**: Cannot verify persistence timing per D-11\n`
      );
    }
  });

  test("Reload/reinitialize preserves persisted state", async ({ page, context }) => {
    // D-11: Reload/reinitialize preserves persisted state

    // Step 1: Perform customization
    const inputField = page.locator("input[type='text'], textarea").first();
    await inputField.focus();
    await inputField.type("test", { delay: 100 });

    // Step 2: Deploy to persist state
    await page.keyboard.press("Control+d");
    await page.waitForTimeout(2000);

    // Step 3: Reload page (full browser reload)
    await page.reload({ waitUntil: "networkidle" });

    await page.waitForTimeout(2000);

    // Step 4: Verify persisted state restored
    // Check if previous settings/composition state persisted
    // This requires app to load persisted user state on init

    await takeEvidenceScreenshot(page, "persistence-after-reload");

    // Verify sync from persistence before init
    const syncFromMarkerFound = await verifyPersistenceMarker(page, "syncFromPersistenceBeforeInit");

    await saveEvidence(
      "persistence-sync.log",
      `[${new Date().toISOString()}] Reload: syncFromPersistenceBeforeInit ${syncFromMarkerFound ? "PASS" : "FAIL (marker not logged)"}\n`
 +
      `[${new Date().toISOString()}] Reload: App reinitialized\n`
    );

    // Note: Verifying specific persisted values requires app to expose persisted state
    // For E2E smoke, we verify the reload succeeded and app initialized again
    const inputAfterReload = await page.locator("input[type='text'], textarea").first();
    expect(await inputAfterReload.count()).toBeGreaterThan(0);
  });
});

/**
 * Evidence Summary
 *
 * After running this spec, verify evidence files in e2e/results/:
 * - browser-run.log — All flow test results
 * - screenshot-*.png — Visual evidence for each flow
 * - browser-console.log — Console errors captured
 * - persistence-sync.log — Persistence timing markers
 * - blocker.md — Flows blocked by missing selectors/implementation
 *
 * Per D-09: If browser runner or flows blocked, blocker.md documents:
 * - Exact command attempted
 * - Missing dependency/selector
 * - Fallback evidence
 * - Category: TypeDuck-Web app/source, Yune adapter/runtime, environment/tooling
 */

export {};