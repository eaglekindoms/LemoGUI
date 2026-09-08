# LemoGUI 富文本 TOML 格式（version 1）

文件扩展名：`.toml`。魔数：`format = "lemogui-rich"`。

TOML 不能把 `[[text]]` 嵌进 `[content]`（会变成顶层表），因此内容是重复的 `[[content]]`。

## 根表

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `format` | string | 固定 `"lemogui-rich"` |
| `version` | integer | 当前 `1` |
| `caret` | integer | 可选，默认 `0`（原子下标，含图片） |
| `fonts` | string 数组 | 字库路径；缺省或空则用库内默认字体 |
| `[style]` | 表 | 全局默认文本样式；缺省整表则等同 `TextStyle` 默认值 |

不写入文件：`sel_anchor`、`is_focus`、`dragging`、`scroll`。打开后 `current_style` 取根表 `[style]`。

## 样式字段

`[style]` 与 `[content.style]` 字段相同。每个键都可选：块上未写的键用全局，全局也未写的键用默认值。

| 字段 | 类型 | 默认 |
| --- | --- | --- |
| `bold` / `italic` / `underline` | bool | `false` |
| `size` | float | `16` |
| `color` | `[r, g, b, a]`，0..1 | `[0.0, 0.0, 0.0, 1.0]` |
| `align` | `"left"` \| `"center"` \| `"right"` | `"left"` |
| `font` | integer | `0`（下标进 `fonts`，越界夹到 `0`） |

## `[[content]]`

按文档顺序。每项为文本或图片。

**文本块**

- `text`：string，一段连续同样式字符（可含换行）
- `[content.style]`：可整表省略。省略则该段全部用全局 `[style]`。若写了，只覆盖出现的键。

**图片块**（无 `text`；路径与 Base64 二选一）

- `image`：string，文件系统路径；若值为 `data:image/...;base64,...` 则按内嵌处理
- `image_b64`：string，原文件字节的标准 Base64（可含空白；可带 data URI 前缀）
- `display`：`"inline"` \| `"block"`，缺省 `"block"`
- `width`：float，显示宽度（像素）；缺省按原图（不超过编辑区行宽）。高度按原图比例缩放。

判别：有 `text` → 文本；否则若有 `image_b64` 或 data URI → 内嵌（解码失败则跳过）；否则若有普通 `image` 路径 → 链接；`image` 与 `image_b64` 同时出现以内嵌为准；其它 → 跳过。

相对路径相对 **toml 文件所在目录** 解析。

## 保存

1. 根表 `[style]` 写成文档的 `current_style`（字段写全一次）。
2. 相邻同样式字符合并成一个 `text`。
3. run 与全局逐字段比较：全相同则不写 `[content.style]`，否则只写不同的键。
4. 图片按插入来源写出：链接写 `image`，文件内嵌写 `image_b64`。`display` 仅在 `"inline"` 时写出。手动缩放过的图写 `width`。

## 打开

1. 解析 TOML；`format` 必须为 `"lemogui-rich"`，`version` 必须为 `1`。
2. 根 `[style]` 覆盖到默认样式，得到全局。
3. 每个文本块：`resolved = 全局.overlay(块 style)`，再按字展开。
4. 每个图片块按上面判别写入文档原子。
5. `current_style = 全局`；`caret` 夹到原子长度。

## 示例

```toml
format = "lemogui-rich"
version = 1
caret = 5
fonts = ["/path/SourceHanSansCN-Regular.otf"]

[style]
bold = false
italic = false
underline = false
size = 16.0
color = [0.0, 0.0, 0.0, 1.0]
align = "left"
font = 0

[[content]]
text = "普通正文"

[[content]]
text = "粗体字"

[content.style]
bold = true

[[content]]
image = "img.png"
display = "inline"
width = 200

[[content]]
image_b64 = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg=="
```
