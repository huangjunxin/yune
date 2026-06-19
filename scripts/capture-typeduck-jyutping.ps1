param(
    [ValidateSet("Smoke", "Options", "CompletionCorrection", "SchemaMenu", "UserDb", "All")]
    [string]$Fixture = "Smoke",
    [string]$OracleRoot,
    [string]$Output,
    [string[]]$Inputs,
    [switch]$InternalCapture,
    [switch]$InternalSchemaList,
    [switch]$InternalUserDbProbe,
    [string]$Shared,
    [string]$User,
    [string]$Build,
    [string]$Schema,
    [string[]]$Modules,
    [string]$InputsFile,
    [string]$CasesOutput,
    [string]$DictName = "jyut6ping3",
    [string]$ExportPath,
    [string]$TrainingInput = "nei"
)

$ErrorActionPreference = "Stop"

function Write-Utf8NoBom([string]$Path, [string]$Text) {
    $Dir = Split-Path -Parent $Path
    if (-not [string]::IsNullOrWhiteSpace($Dir)) {
        New-Item -ItemType Directory -Force -Path $Dir | Out-Null
    }
    $Encoding = [System.Text.UTF8Encoding]::new($false)
    [System.IO.File]::WriteAllText($Path, $Text, $Encoding)
}

function Write-JsonFile([string]$Path, $Value) {
    Write-Utf8NoBom $Path (($Value | ConvertTo-Json -Depth 32) + "`n")
}

function Read-JsonFile([string]$Path) {
    Get-Content -LiteralPath $Path -Raw -Encoding UTF8 | ConvertFrom-Json
}

function FullPath([string]$Path) {
    [System.IO.Path]::GetFullPath($Path)
}

function Assert-UnderRoot([string]$Path, [string]$Root) {
    $Full = FullPath $Path
    $FullRoot = FullPath $Root
    if (-not $Full.StartsWith($FullRoot, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "Refusing to operate outside capture root: $Full"
    }
    $Full
}

function Remove-CaptureTree([string]$Path, [string]$Root) {
    $Full = Assert-UnderRoot $Path $Root
    if (Test-Path -LiteralPath $Full) {
        Remove-Item -LiteralPath $Full -Recurse -Force
    }
}

function Write-CommonCustom([string]$SharedDir, [string[]]$Patches) {
    if ($null -eq $Patches -or $Patches.Count -eq 0) {
        $Text = "patch:`n  __patch: []`n"
    } else {
        $Lines = @("patch:", "  __patch:")
        foreach ($Patch in $Patches) {
            $Lines += "    - $Patch"
        }
        $Text = ($Lines -join "`n") + "`n"
    }
    Write-Utf8NoBom (Join-Path $SharedDir "common.custom.yaml") $Text
}

function Assert-OracleInputs {
    $RequiredPaths = @(
        (Join-Path $Extract "dist\lib\rime.dll"),
        (Join-Path $Extract "dist\bin\rime_deployer.exe"),
        (Join-Path $Extract "dist\include\rime_api.h"),
        (Join-Path $SharedBase "jyut6ping3_mobile.schema.yaml"),
        (Join-Path $SharedBase "jyut6ping3.schema.yaml"),
        (Join-Path $SharedBase "jyut6ping3.dict.yaml"),
        (Join-Path $RepoRoot "scripts\oracle-rime-probe.cs")
    )
    foreach ($Path in $RequiredPaths) {
        if (-not (Test-Path -LiteralPath $Path)) {
            throw "Missing required TypeDuck oracle input: $Path"
        }
    }
}

function Get-SchemaCommit {
    $SchemaSource = Get-ChildItem -LiteralPath $SchemaRoot -Directory -Filter "schema-*" |
        Select-Object -First 1
    if ($null -eq $SchemaSource) {
        throw "Missing TypeDuck schema source checkout under $SchemaRoot"
    }
    $Commit = $SchemaSource.Name -replace "^schema-", ""
    if ($Commit.Length -ne 40) {
        throw "Unexpected TypeDuck schema source directory name: $($SchemaSource.Name)"
    }
    $Commit
}

function Invoke-Deployer([string]$VariantUser, [string]$VariantShared, [string]$VariantBuild) {
    New-Item -ItemType Directory -Force -Path $VariantBuild | Out-Null
    & (Join-Path $Extract "dist\bin\rime_deployer.exe") --build $VariantUser $VariantShared $VariantBuild
    if ($LASTEXITCODE -ne 0) {
        throw "rime_deployer.exe --build failed with exit code $LASTEXITCODE"
    }
}

function New-Variant(
    [string]$Group,
    [string]$Name,
    [string]$VariantSchema,
    [string[]]$Patches,
    [string[]]$VariantInputs,
    [string]$DefaultCustomMode = "Keep"
) {
    $Root = Join-Path $CaptureRoot (Join-Path $Group $Name)
    Remove-CaptureTree $Root $CaptureRoot
    $VariantShared = Join-Path $Root "shared"
    $VariantUser = Join-Path $Root "user"
    $VariantBuild = Join-Path $VariantUser "build"
    New-Item -ItemType Directory -Force -Path $Root, $VariantUser | Out-Null
    Copy-Item -LiteralPath $SharedBase -Destination $VariantShared -Recurse
    Write-CommonCustom $VariantShared $Patches
    if ($DefaultCustomMode -eq "Remove") {
        $DefaultCustom = Join-Path $VariantShared "default.custom.yaml"
        if (Test-Path -LiteralPath $DefaultCustom) {
            Remove-Item -LiteralPath $DefaultCustom -Force
        }
    }
    Invoke-Deployer $VariantUser $VariantShared $VariantBuild
    [ordered]@{
        group = $Group
        name = $Name
        schema = $VariantSchema
        patches = $Patches
        inputs = $VariantInputs
        root = $Root
        shared = $VariantShared
        user = $VariantUser
        build = $VariantBuild
        default_custom_mode = $DefaultCustomMode
    }
}

function Invoke-ChildCapture($Variant) {
    $CasesPath = Join-Path $Variant.root "cases.json"
    $InputsPath = Join-Path $Variant.root "inputs.json"
    Write-JsonFile $InputsPath $Variant.inputs
    $Args = @(
        "-NoProfile",
        "-ExecutionPolicy", "Bypass",
        "-File", $PSCommandPath,
        "-InternalCapture",
        "-OracleRoot", $OracleRoot,
        "-Shared", $Variant.shared,
        "-User", $Variant.user,
        "-Build", $Variant.build,
        "-Schema", $Variant.schema,
        "-CasesOutput", $CasesPath,
        "-Modules", ($Modules -join ","),
        "-InputsFile", $InputsPath
    )
    & powershell @Args
    if ($LASTEXITCODE -ne 0) {
        throw "child capture failed for $($Variant.name) with exit code $LASTEXITCODE"
    }
    $RawCases = Get-Content -LiteralPath $CasesPath -Raw -Encoding UTF8 | ConvertFrom-Json
    $CaseList = if ($RawCases -is [System.Array]) { $RawCases } else { @($RawCases) }
    foreach ($_ in $CaseList) {
        $Row = [ordered]@{}
        foreach ($Property in $_.PSObject.Properties) {
            $Row[$Property.Name] = $Property.Value
        }
        $Row["variant"] = $Variant.name
        $Row["variant_schema"] = $Variant.schema
        $Row["patches"] = $Variant.patches
        $Row["default_custom_mode"] = $Variant.default_custom_mode
        [pscustomobject]$Row
    }
}

function Invoke-ChildSchemaList($Variant) {
    $CasesPath = Join-Path $Variant.root "schema-list.json"
    $Args = @(
        "-NoProfile",
        "-ExecutionPolicy", "Bypass",
        "-File", $PSCommandPath,
        "-InternalSchemaList",
        "-OracleRoot", $OracleRoot,
        "-Shared", $Variant.shared,
        "-User", $Variant.user,
        "-Build", $Variant.build,
        "-CasesOutput", $CasesPath,
        "-Modules", ($Modules -join ",")
    )
    & powershell @Args
    if ($LASTEXITCODE -ne 0) {
        throw "schema-list capture failed for $($Variant.name) with exit code $LASTEXITCODE"
    }
    $Result = Read-JsonFile $CasesPath
    [ordered]@{
        variant = $Variant.name
        patches = $Variant.patches
        default_custom_mode = $Variant.default_custom_mode
        rime_get_schema_list = $Result.rime_get_schema_list
        schemas = $Result.schemas
    }
}

function Invoke-ChildUserDbProbe($Variant) {
    $CasesPath = Join-Path $Variant.root "userdb-probe.json"
    $Export = Join-Path $Variant.root "jyut6ping3-userdb-export.txt"
    $Args = @(
        "-NoProfile",
        "-ExecutionPolicy", "Bypass",
        "-File", $PSCommandPath,
        "-InternalUserDbProbe",
        "-OracleRoot", $OracleRoot,
        "-Shared", $Variant.shared,
        "-User", $Variant.user,
        "-Build", $Variant.build,
        "-Schema", $Variant.schema,
        "-CasesOutput", $CasesPath,
        "-DictName", $DictName,
        "-ExportPath", $Export,
        "-TrainingInput", $TrainingInput,
        "-Modules", ($Modules -join ",")
    )
    & powershell @Args
    if ($LASTEXITCODE -ne 0) {
        throw "userdb probe failed for $($Variant.name) with exit code $LASTEXITCODE"
    }
    Read-JsonFile $CasesPath
}

function New-Fixture([string]$SourceRowPolicy, [string[]]$Schemas, [string[]]$Scenarios, $Cases, $ExtraCapture) {
    $Capture = [ordered]@{
        schema_data = "TypeDuck-HK/schema"
        schema_data_commit = $SchemaCommit
        dictionary = "jyut6ping3.dict.yaml"
        source_row_policy = $SourceRowPolicy
        candidate_layout = "TypeDuck v1.1.2 RimeCandidate includes quality before reserved"
        option_delivery = "deploy-time common.custom.yaml/default.custom.yaml variants; runtime set_option does not apply these schema __patch hooks"
        modules = $Modules
    }
    if ($null -ne $ExtraCapture) {
        foreach ($Property in $ExtraCapture.GetEnumerator()) {
            $Capture[$Property.Key] = $Property.Value
        }
    }

    [ordered]@{
        oracle = [ordered]@{
            engine = "TypeDuck-HK/librime"
            engine_tag = "v1.1.2"
            engine_commit = "74cb52b78fb2411137a7643f6c8bc6517acfde69"
            canonical_repository = "https://github.com/TypeDuck-HK/librime"
            release_url = "https://github.com/TypeDuck-HK/librime/releases/tag/v1.1.2"
            release_asset = "rime-TypeDuck-v1.1.2-Windows-msvc-x64.7z"
            dependency_asset = "rime-deps-TypeDuck-v1.1.2-Windows-msvc-x64.7z"
            plugin = "TypeDuck-HK/rime-dictionary-lookup-filter"
            plugin_commit = "3e4605c4fae99f068df2edb85aaeab5a97752795"
            schema = "TypeDuck-HK/schema"
            schema_commit = $SchemaCommit
            capture_date = (Get-Date -Format "yyyy-MM-dd")
            capture_command = "powershell -ExecutionPolicy Bypass -File scripts/capture-typeduck-jyutping.ps1 -Fixture $Fixture"
            host = "Windows"
        }
        schema = if ($Schemas.Count -eq 1) { $Schemas[0] } else { "mixed" }
        schemas = $Schemas
        module_list = $Modules
        scenarios = $Scenarios
        capture = $Capture
        cases = @($Cases)
    }
}

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
if ([string]::IsNullOrWhiteSpace($OracleRoot)) {
    $OracleRoot = Join-Path $RepoRoot "target\typeduck-oracle\v1.1.2"
}
$OracleRoot = FullPath $OracleRoot
$Extract = Join-Path $OracleRoot "extract"
$SharedBase = Join-Path $OracleRoot "rime-shared"
$UserBase = Join-Path $OracleRoot "rime-user"
$BuildBase = Join-Path $UserBase "build"
$SchemaRoot = Join-Path $OracleRoot "schema-src"
$CaptureRoot = Join-Path $OracleRoot "m14-capture"
if ($null -eq $Modules -or $Modules.Count -eq 0) {
    $Modules = [string[]]@("default", "dictionary_lookup")
}
if ($Modules.Count -eq 1 -and $Modules[0].Contains(",")) {
    $Modules = [string[]]($Modules[0].Split(","))
}
if (-not [string]::IsNullOrWhiteSpace($InputsFile)) {
    $RawInputs = Get-Content -LiteralPath $InputsFile -Raw -Encoding UTF8 | ConvertFrom-Json
    $Inputs = if ($RawInputs -is [System.Array]) { [string[]]$RawInputs } else { [string[]]@($RawInputs) }
}
if ($null -ne $Inputs -and $Inputs.Count -eq 1 -and $Inputs[0].Contains(",")) {
    $Inputs = [string[]]($Inputs[0].Split(","))
}

$env:PATH = (Join-Path $Extract "dist\lib") + ";" + (Join-Path $Extract "bin") + ";" + $env:PATH

if ($InternalCapture -or $InternalSchemaList -or $InternalUserDbProbe) {
    Add-Type -Path (Join-Path $RepoRoot "scripts\oracle-rime-probe.cs")
    $Identity = [RimeProbe]::TypeDuckV112Identity()
    if ($InternalCapture) {
        if ($null -eq $Inputs -or $Inputs.Count -eq 0) {
            throw "Internal capture requires -Inputs"
        }
        $Cases = [RimeProbe]::CaptureWithIdentity(
            $Shared,
            $User,
            $Build,
            $Schema,
            [string[]]$Modules,
            [string[]]$Inputs,
            $Identity
        )
        Write-JsonFile $CasesOutput $Cases
    } elseif ($InternalSchemaList) {
        $Result = [RimeProbe]::CaptureSchemaListWithIdentity(
            $Shared,
            $User,
            $Build,
            [string[]]$Modules,
            $Identity
        )
        Write-JsonFile $CasesOutput $Result
    } else {
        $Result = [RimeProbe]::ProbeUserDictExportWithIdentity(
            $Shared,
            $User,
            $Build,
            $Schema,
            [string[]]$Modules,
            $TrainingInput,
            $DictName,
            $ExportPath,
            $Identity
        )
        Write-JsonFile $CasesOutput $Result
    }
    exit 0
}

Assert-OracleInputs
$SchemaCommit = Get-SchemaCommit
New-Item -ItemType Directory -Force -Path $CaptureRoot | Out-Null

if ($Fixture -eq "All") {
    foreach ($Name in @("Smoke", "Options", "CompletionCorrection", "SchemaMenu", "UserDb")) {
        & $PSCommandPath -Fixture $Name -OracleRoot $OracleRoot
        if ($LASTEXITCODE -ne 0) {
            throw "fixture capture failed for $Name with exit code $LASTEXITCODE"
        }
    }
    exit 0
}

if ([string]::IsNullOrWhiteSpace($Output)) {
    $OutputName = switch ($Fixture) {
        "Smoke" { "jyut6ping3-m14-smoke.json" }
        "Options" { "jyut6ping3-m14-options.json" }
        "CompletionCorrection" { "jyut6ping3-m14-completion-correction.json" }
        "SchemaMenu" { "jyut6ping3-m14-schema-menu.json" }
        "UserDb" { "jyut6ping3-m14-userdb.json" }
    }
    $Output = Join-Path $RepoRoot (Join-Path "crates\yune-core\tests\fixtures\typeduck-v1.1.2" $OutputName)
}
$Output = FullPath $Output

if ($Fixture -eq "Smoke") {
    if ($null -eq $Inputs -or $Inputs.Count -eq 0) {
        $Inputs = [string[]]@("nei")
    }
    Invoke-Deployer $UserBase $SharedBase $BuildBase
    $Variant = [ordered]@{
        name = "smoke"
        schema = "jyut6ping3_mobile"
        patches = @()
        inputs = $Inputs
        root = Join-Path $CaptureRoot "smoke"
        shared = $SharedBase
        user = $UserBase
        build = $BuildBase
        default_custom_mode = "Keep"
    }
    New-Item -ItemType Directory -Force -Path $Variant.root | Out-Null
    $Cases = Invoke-ChildCapture $Variant
    $FixtureBody = New-Fixture "typeduck_v112_binary_smoke" @("jyut6ping3_mobile") @("smoke") $Cases ([ordered]@{ input_sequence = $Inputs })
    $FixtureBody["input_sequence"] = $Inputs
    Write-JsonFile $Output $FixtureBody
} elseif ($Fixture -eq "Options") {
    $Grave = [string][char]96
    $Variants = @(
        (New-Variant -Group "options" -Name "combine_candidates_default" -VariantSchema "jyut6ping3_mobile" -Patches ([string[]]@()) -VariantInputs ([string[]]@("hou", "nei"))),
        (New-Variant -Group "options" -Name "combine_candidates_separate" -VariantSchema "jyut6ping3_mobile" -Patches ([string[]]@("common:/separate_candidates")) -VariantInputs ([string[]]@("hou", "nei"))),
        (New-Variant -Group "options" -Name "enable_sentence_default" -VariantSchema "jyut6ping3_mobile" -Patches ([string[]]@()) -VariantInputs ([string[]]@("ngohaigo", "ngohaige"))),
        (New-Variant -Group "options" -Name "enable_sentence_disabled" -VariantSchema "jyut6ping3_mobile" -Patches ([string[]]@("common:/disable_sentence")) -VariantInputs ([string[]]@("ngohaigo", "ngohaige"))),
        (New-Variant -Group "options" -Name "show_full_code_default" -VariantSchema "jyut6ping3" -Patches ([string[]]@()) -VariantInputs ([string[]]@(($Grave + "ca"), ($Grave + "cam"), ($Grave + "cd"))) -DefaultCustomMode "Remove"),
        (New-Variant -Group "options" -Name "show_full_code_enabled" -VariantSchema "jyut6ping3" -Patches ([string[]]@("common:/show_full_code")) -VariantInputs ([string[]]@(($Grave + "ca"), ($Grave + "cam"), ($Grave + "cd"))) -DefaultCustomMode "Remove")
    )
    $Cases = foreach ($Variant in $Variants) { Invoke-ChildCapture $Variant }
    $FixtureBody = New-Fixture "typeduck_v112_deploy_time_option_variants" @("jyut6ping3_mobile", "jyut6ping3") @("combine_candidates", "show_full_code", "enable_sentence") $Cases ([ordered]@{})
    Write-JsonFile $Output $FixtureBody
} elseif ($Fixture -eq "CompletionCorrection") {
    $Variants = @(
        (New-Variant -Group "completion-correction" -Name "completion_default" -VariantSchema "jyut6ping3_mobile" -Patches ([string[]]@()) -VariantInputs ([string[]]@("n", "ne", "ng"))),
        (New-Variant -Group "completion-correction" -Name "completion_disabled" -VariantSchema "jyut6ping3_mobile" -Patches ([string[]]@("common:/disable_completion")) -VariantInputs ([string[]]@("n", "ne", "ng"))),
        (New-Variant -Group "completion-correction" -Name "correction_default" -VariantSchema "jyut6ping3_mobile" -Patches ([string[]]@()) -VariantInputs ([string[]]@("nri", "mgoi", "ngoi"))),
        (New-Variant -Group "completion-correction" -Name "correction_enabled" -VariantSchema "jyut6ping3_mobile" -Patches ([string[]]@("common:/enable_correction")) -VariantInputs ([string[]]@("nri", "mgoi", "ngoi")))
    )
    $Cases = foreach ($Variant in $Variants) { Invoke-ChildCapture $Variant }
    $FixtureBody = New-Fixture "typeduck_v112_deploy_time_completion_correction_variants" @("jyut6ping3_mobile") @("enable_completion", "enable_correction") $Cases ([ordered]@{})
    Write-JsonFile $Output $FixtureBody
} elseif ($Fixture -eq "SchemaMenu") {
    $Variants = @(
        (New-Variant -Group "schema-menu" -Name "one_schema_default" -VariantSchema "jyut6ping3" -Patches ([string[]]@()) -VariantInputs ([string[]]@("nei")) -DefaultCustomMode "Remove"),
        (New-Variant -Group "schema-menu" -Name "mobile_multi_schema_custom" -VariantSchema "jyut6ping3_mobile" -Patches ([string[]]@()) -VariantInputs ([string[]]@("nei")) -DefaultCustomMode "Keep")
    )
    $Cases = foreach ($Variant in $Variants) { [pscustomobject](Invoke-ChildSchemaList $Variant) }
    $Finding = [ordered]@{
        oracle_observable_surface = "RimeGetSchemaList emits selected schema rows; hide_lone_schema/hide_caret are switcher/frontend decoration, not candidate ABI rows"
        m16_delivery = "assert TypeDuck-Web UI behavior against one-schema and multi-schema emitted lists"
    }
    $FixtureBody = New-Fixture "typeduck_v112_schema_list_emitted_surface" @("jyut6ping3", "jyut6ping3_mobile") @("schema_menu_hiding") $Cases $Finding
    Write-JsonFile $Output $FixtureBody
} elseif ($Fixture -eq "UserDb") {
    $Modules = [string[]]@("default", "dictionary_lookup", "levers")
    $Variant = New-Variant -Group "userdb" -Name "levers_export_probe" -VariantSchema "jyut6ping3_mobile" -Patches ([string[]]@()) -VariantInputs ([string[]]@($TrainingInput))
    $Result = Invoke-ChildUserDbProbe $Variant
    $Cases = @([ordered]@{
        variant = $Variant.name
        schema_id = $Variant.schema
        training_input = $TrainingInput
        probe = $Result
    })
    $Finding = [ordered]@{
        feasibility_spike = "trained one highlighted candidate, called RimeSyncUserData, then tried RimeFindModule('levers') and RimeLeversApi.export_user_dict"
        dict_name = $DictName
    }
    $FixtureBody = New-Fixture "typeduck_v112_userdb_levers_export_probe" @("jyut6ping3_mobile") @("per_entry_userdb_pronunciation") $Cases $Finding
    Write-JsonFile $Output $FixtureBody
}

Write-Host "Wrote $Output"
