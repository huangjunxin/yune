import { defineConfig } from "@playwright/test";

export default defineConfig({
  testDir: ".",
  testMatch: "yune-typeduck.spec.ts",
  reporter: "line",
  use: {
    baseURL: process.env.TYPEDUCK_APP_URL || "http://localhost:5173",
  },
});
