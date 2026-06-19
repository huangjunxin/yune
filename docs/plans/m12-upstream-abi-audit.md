# M12 Upstream ABI Audit

> **Status:** Active - **Milestone:** M12 (Upstream Oracle Refresh) - **Updated:** 2026-06-19 - **Type:** reference

Upstream source: `rime/librime` tag `1.17.0`, commit
`33e78140250125871856cdc5b42ddc6a5fcd3cd4`, file `src/rime_api.h`.

| Field | Upstream 1.17.0 slot | Current Yune slot before M12 | M12 classification | Action |
|---|---:|---:|---|---|
| `start_maintenance` | 4 | 4 | upstream core | keep |
| `start_quick` | absent | absent | TypeDuck profile | keep absent from default `RimeApi`; any future support must remain profile-only |
| `is_maintenance_mode` | 5 | 5 | upstream core | keep |
| `config_begin_map` | 44 | 44 | upstream core | keep |
| `config_list_append_bool` | absent | 68 | TypeDuck profile | remove from default `RimeApi`; keep direct helper tests |
| `config_list_append_int` | absent | 69 | TypeDuck profile | remove from default `RimeApi`; keep direct helper tests |
| `config_list_append_double` | absent | 70 | TypeDuck profile | remove from default `RimeApi`; keep direct helper tests |
| `config_list_append_string` | absent | 71 | TypeDuck profile | remove from default `RimeApi`; keep direct helper tests |
| `config_begin_list` | 68 | 72 | upstream core | move to upstream slot 68 |
| `get_input` | 69 | 73 | upstream core | move to upstream slot 69 |
| `get_prebuilt_data_dir` | 80 | 84 | upstream core | move to upstream slot 80 |
| `get_staging_dir` | 81 | 85 | upstream core | move to upstream slot 81 |
| `change_page` | 97 | 101 | upstream core | move to upstream slot 97 |
| function slot count | 98 | 102 | upstream core | default `RimeApi` exposes 98 function slots |
