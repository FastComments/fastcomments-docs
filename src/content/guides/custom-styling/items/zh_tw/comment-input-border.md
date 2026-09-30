The border around the comment box is made of the text area's own border plus a few thin lines the widget draws around it. To change its color or corner roundness, set these CSS variables instead of styling the `textarea` directly. The variables restyle every part of the border at once, so the corners and colors always line up.

| 變數 | 變更內容 | 預設 |
|---|---|---|
| `--fc-input-border-color` | 邊框顏色 | `#bfbfbf` |
| `--fc-input-border-color-focus` | 使用者輸入時的邊框顏色 | `#555` |
| `--fc-input-border-radius` | 圓角的圓潤程度 | `11px` |
| `--fc-input-border-start-start-radius` | 左上角（在從右至左語言中為右上角）方形角的圓潤程度 | `0` |

Add the CSS to the **Custom CSS** box on the [Widget Customization page](https://fastcomments.com/auth/my-account/customize-widget), or pass it with the `customCSS` option. You only need to set the variables you want to change.

You can also use the **Comment box border** helper right under the Custom CSS box, which writes this CSS for you.

## 變更邊框顏色

[inline-code-attrs-start title = '邊框顏色'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## 輸入時變更邊框顏色

[inline-code-attrs-start title = '輸入時的邊框顏色'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## 圓化所有四個角

By default the top-left corner is square. Set both radius variables to round all four corners the same:

[inline-code-attrs-start title = '圓化所有四個角'; type = 'css'; isFunctional = false; inline-code-attrs-end]
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

## 與您的品牌相符

[inline-code-attrs-start title = '品牌顏色與角落'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## 暗色模式下的不同顏色

When the widget is in dark mode it has the `dark` class, so you can set different values for dark mode:

[inline-code-attrs-start title = '暗色模式邊框顏色'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## 為何不直接為文字區域設定樣式？

The widget draws part of the comment box border itself, around the text area. If you set `border-color` or `border-radius` only on the `textarea`, those lines keep the default style and the border looks mismatched, for example a square line running through a rounded corner. The variables above change both at once.