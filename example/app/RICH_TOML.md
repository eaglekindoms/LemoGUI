# LemoGUI 富文本 TOML 格式（version 1）

文件扩展名：`.toml`。魔数：`format = "lemogui-rich"`。

图片字段只占位：打开时跳过，文档模型仍是字符序列。TOML 不能把 `[[text]]` 嵌进 `[content]`（会变成顶层表），因此内容是重复的 `[[content]]`。

## 根表

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `format` | string | 固定 `"lemogui-rich"` |
| `version` | integer | 当前 `1` |
| `caret` | integer | 可选，默认 `0` |
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

按文档顺序。每项二选一：

**文本块**

- `text`：string，一段连续同样式字符（可含换行）
- `[content.style]`：可整表省略。省略则该段全部用全局 `[style]`。若写了，只覆盖出现的键。

**图片块**

- `image`：string，图片路径。打开时丢弃整块。

判别：有 `text` → 文本；无 `text` 且有 `image` → 跳过；其它 → 跳过。

## 保存

1. 根表 `[style]` 写成文档的 `current_style`（字段写全一次）。
2. 相邻同样式字符合并成一个 `text`。
3. run 与全局逐字段比较：全相同则不写 `[content.style]`，否则只写不同的键。
4. 当前没有图片可写。

## 打开

1. 解析 TOML；`format` 必须为 `"lemogui-rich"`，`version` 必须为 `1`。
2. 根 `[style]` 覆盖到默认样式，得到全局。
3. 每个文本块：`resolved = 全局.overlay(块 style)`，再按字展开。
4. `current_style = 全局`；`caret` 夹到字符长度。

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
```
