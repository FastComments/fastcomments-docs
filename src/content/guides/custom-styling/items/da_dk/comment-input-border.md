The border around the comment box is made of the text area's own border plus a few thin lines the widget draws around it. To change its color or corner roundness, set these CSS variables instead of styling the `textarea` directly. The variables restyle every part of the border at once, so the corners and colors always line up.

| Variable | Hvad den ændrer | Standard |
|---|---|---|
| `--fc-input-border-color` | Kantfarve | `#bfbfbf` |
| `--fc-input-border-color-focus` | Kantfarve mens brugeren skriver | `#555` |
| `--fc-input-border-radius` | Rundhed af de afrundede hjørner | `11px` |
| `--fc-input-border-start-start-radius` | Rundhed af det firkantede øverste venstre hjørne (øverste højre i højre-til-venstre sprog) | `0` |

Add the CSS to the **Custom CSS** box on the [Widget Customization page](https://fastcomments.com/auth/my-account/customize-widget), or pass it with the `customCSS` option. You only need to set the variables you want to change.

You can also use the **Comment box border** helper right under the Custom CSS box, which writes this CSS for you.

## Skift kantfarven

[inline-code-attrs-start title = 'Kantfarve'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## Skift kantfarve under indtastning

[inline-code-attrs-start title = 'Kantfarve under indtastning'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## Rund alle fire hjørner

By default the top-left corner is square. Set both radius variables to round all four corners the same:

[inline-code-attrs-start title = 'Rund alle fire hjørner'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## Firkantede hjørner

[inline-code-attrs-start title = 'Firkantede hjørner'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## Tilpas til dit brand

[inline-code-attrs-start title = 'Brandfarver og hjørner'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## Forskellige farver i mørk tilstand

When the widget is in dark mode it has the `dark` class, so you can set different values for dark mode:

[inline-code-attrs-start title = 'Mørk tilstand kantfarver'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## Hvorfor ikke style tekstområdet direkte?

The widget draws part of the comment box border itself, around the text area. If you set `border-color` or `border-radius` only on the `textarea`, those lines keep the default style and the border looks mismatched, for example a square line running through a rounded corner. The variables above change both at once.