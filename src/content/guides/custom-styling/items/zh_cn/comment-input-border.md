The border around the comment box is made of the text area's own border plus a few thin lines the widget draws around it. To change its color or corner roundness, set these CSS variables instead of styling the `textarea` directly. The variables restyle every part of the border at once, so the corners and colors always line up.

| 变量 | 更改内容 | 默认 |
|---|---|---|
| `--fc-input-border-color` | 边框颜色 | `#bfbfbf` |
| `--fc-input-border-color-focus` | 用户输入时的边框颜色 | `#555` |
| `--fc-input-border-radius` | 圆角的圆润程度 | `11px` |
| `--fc-input-border-start-start-radius` | 方形左上角（在从右到左的语言中为右上角）的圆润程度 | `0` |

Add the CSS to the **自定义 CSS** box on the [Widget Customization page](https://fastcomments.com/auth/my-account/customize-widget), or pass it with the `customCSS` option. You only need to set the variables you want to change.

You can also use the **Comment box border** helper right under the Custom CSS box, which writes this CSS for you.

## 更改边框颜色

[inline-code-attrs-start title = '边框颜色'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## 更改输入时的边框颜色

[inline-code-attrs-start title = '输入时的边框颜色'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## 圆化四个角

By default the top-left corner is square. Set both radius variables to round all four corners the same:

[inline-code-attrs-start title = '圆化四个角'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## 方形角

[inline-code-attrs-start title = '方形角'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## 匹配品牌风格

[inline-code-attrs-start title = '品牌颜色和圆角'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## 暗模式下的不同颜色

When the widget is in dark mode it has the `dark` class, so you can set different values for dark mode:

[inline-code-attrs-start title = '暗模式边框颜色'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## 为什么不直接为文本区域设置样式？

The widget draws part of the comment box border itself, around the text area. If you set `border-color` or `border-radius` only on the `textarea`, those lines keep the default style and the border looks mismatched, for example a square line running through a rounded corner. The variables above change both at once.

---