The border around the comment box is made of the text area's own border plus a few thin lines the widget draws around it. To change its color or corner roundness, set these CSS variables instead of styling the `textarea` directly. The variables restyle every part of the border at once, so the corners and colors always line up.

| Variable | What it changes | Default |
|---|---|---|
| `--fc-input-border-color` | Колір рамки | `#bfbfbf` |
| `--fc-input-border-color-focus` | Колір рамки під час вводу користувачем | `#555` |
| `--fc-input-border-radius` | Закругленість закруглених кутів | `11px` |
| `--fc-input-border-start-start-radius` | Закругленість квадратного верхнього лівого кута (верхнього правого у мовах з напрямком справа наліво) | `0` |

Add the CSS to the **Custom CSS** box on the [Widget Customization page](https://fastcomments.com/auth/my-account/customize-widget), or pass it with the `customCSS` option. You only need to set the variables you want to change.

You can also use the **Comment box border** helper right under the Custom CSS box, which writes this CSS for you.

## Змінити колір рамки

[inline-code-attrs-start title = 'Колір рамки'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## Змінити колір рамки під час вводу

[inline-code-attrs-start title = 'Колір рамки під час вводу'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## Закруглити всі чотири кути

By default the top-left corner is square. Set both radius variables to round all four corners the same:

[inline-code-attrs-start title = 'Закруглити всі чотири кути'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## Квадратні кути

[inline-code-attrs-start title = 'Квадратні кути'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## Підлаштувати під ваш бренд

[inline-code-attrs-start title = 'Кольори бренду та кути'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## Різні кольори в темному режимі

When the widget is in dark mode it has the `dark` class, so you can set different values for dark mode:

[inline-code-attrs-start title = 'Кольори рамки в темному режимі'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## Чому не стилізувати текстову область безпосередньо?

The widget draws part of the comment box border itself, around the text area. If you set `border-color` or `border-radius` only on the `textarea`, those lines keep the default style and the border looks mismatched, for example a square line running through a rounded corner. The variables above change both at once.

---