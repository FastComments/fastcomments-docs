The border around the comment box is made of the text area's own border plus a few thin lines the widget draws around it. To change its color or corner roundness, set these CSS variables instead of styling the `textarea` directly. The variables restyle every part of the border at once, so the corners and colors always line up.

| 変数 | 変更内容 | デフォルト |
|---|---|---|
| `--fc-input-border-color` | 枠線の色 | `#bfbfbf` |
| `--fc-input-border-color-focus` | ユーザーが入力中の枠線の色 | `#555` |
| `--fc-input-border-radius` | 角の丸み | `11px` |
| `--fc-input-border-start-start-radius` | 正方形の左上隅の丸み（右から左の言語では右上） | `0` |

Add the CSS to the **Custom CSS** box on the [Widget Customization page](https://fastcomments.com/auth/my-account/customize-widget), or pass it with the `customCSS` option. You only need to set the variables you want to change.

You can also use the **Comment box border** helper right under the Custom CSS box, which writes this CSS for you.

## Change the Border Color

[inline-code-attrs-start title = '枠線の色'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## Change the Border Color While Typing

[inline-code-attrs-start title = '入力中の枠線の色'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## Round All Four Corners

By default the top-left corner is square. Set both radius variables to round all four corners the same:

[inline-code-attrs-start title = 'すべての角を丸くする'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## Square Corners

[inline-code-attrs-start title = '角を四角くする'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## Match Your Brand

[inline-code-attrs-start title = 'ブランドの色と角'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## Different Colors in Dark Mode

When the widget is in dark mode it has the `dark` class, so you can set different values for dark mode:

[inline-code-attrs-start title = 'ダークモードの枠線の色'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## Why Not Style the Text Area Directly?

The widget draws part of the comment box border itself, around the text area. If you set `border-color` or `border-radius` only on the `textarea`, those lines keep the default style and the border looks mismatched, for example a square line running through a rounded corner. The variables above change both at once.

---