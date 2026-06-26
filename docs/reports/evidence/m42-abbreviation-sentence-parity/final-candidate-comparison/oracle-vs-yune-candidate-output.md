# M42 Oracle vs Yune Candidate Output

Oracle: native upstream rime/librime 1.17.0 with `luna_pinyin`. Product/browser/TypeDuck rows are not used.

## cszysmsrsd

- First-page candidate text/order/comments: True
- Context preedit: True (c s z y s m s r s d)
- Commit preview: True (重商主義什麼是認識到)
- Page metadata: True (page_size=5, page_no=0, is_last_page=False)
- RimeGetInput match: False (oracle=c s z y s m s r s d, Yune=cszysmsrsd; Yune keeps the raw keystroke buffer while context preedit carries segmentation.)

| index | oracle | yune | comment |
| --- | --- | --- | --- |
| 0 | 重商主義什麼是認識到 | 重商主義什麼是認識到 |  |
| 1 | 重商主義 | 重商主義 |  |
| 2 | 催生作用 | 催生作用 |  |
| 3 | 產生爭議 | 產生爭議 |  |
| 4 | 測試資源 | 測試資源 |  |

## zybfshmsru

- First-page candidate text/order/comments: True
- Context preedit: True (z y b f sh m s ru)
- Commit preview: True (自有辦法什麼收入)
- Page metadata: True (page_size=5, page_no=0, is_last_page=False)
- RimeGetInput match: False (oracle=z y b f sh m s ru, Yune=zybfshmsru; Yune keeps the raw keystroke buffer while context preedit carries segmentation.)

| index | oracle | yune | comment |
| --- | --- | --- | --- |
| 0 | 自有辦法什麼收入 | 自有辦法什麼收入 |  |
| 1 | 自有辦法 | 自有辦法 |  |
| 2 | 重要部分 | 重要部分 |  |
| 3 | 晝夜不分 | 晝夜不分 |  |
| 4 | 主要部分 | 主要部分 |  |
