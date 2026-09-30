The border around the comment box is made of the text area's own border plus a few thin lines the widget draws around it. To change its color or corner roundness, set these CSS variables instead of styling the `textarea` directly. The variables restyle every part of the border at once, so the corners and colors always line up.

| Променлива | Какво променя | По подразбиране |
|---|---|---|
| `--fc-input-border-color` | Цвят на границата | `#bfbfbf` |
| `--fc-input-border-color-focus` | Цвят на границата, докато потребителят пише | `#555` |
| `--fc-input-border-radius` | Закръгленост на закръглените ъгли | `11px` |
| `--fc-input-border-start-start-radius` | Закръгленост на квадратния горен ляв ъгъл (горен десен в езици с писане отдясно наляво) | `0` |

Add the CSS to the **Custom CSS** box on the [страница за персонализиране на уиджета](https://fastcomments.com/auth/my-account/customize-widget), or pass it with the `customCSS` option. You only need to set the variables you want to change.

You can also use the **Comment box border** helper right under the Custom CSS box, which writes this CSS for you.

## Промяна на цвета на границата

[inline-code-attrs-start title = 'Цвят на границата'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## Промяна на цвета на границата докато се пише

[inline-code-attrs-start title = 'Цвят на границата докато се пише'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## Закръгляне на всички четири ъгъла

By default the top-left corner is square. Set both radius variables to round all four corners the same:

[inline-code-attrs-start title = 'Закръгляне на всички четири ъгъла'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## Квадратни ъгли

[inline-code-attrs-start title = 'Квадратни ъгли'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## Съответства на вашата марка

[inline-code-attrs-start title = 'Цветове и ъгли на марката'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## Различни цветове в тъмен режим

When the widget is in dark mode it has the `dark` class, so you can set different values for dark mode:

[inline-code-attrs-start title = 'Цветове на границата в тъмен режим'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## Защо да не стилизираме директно текстовото поле?

The widget draws part of the comment box border itself, around the text area. If you set `border-color` or `border-radius` only on the `textarea`, those lines keep the default style and the border looks mismatched, for example a square line running through a rounded corner. The variables above change both at once.