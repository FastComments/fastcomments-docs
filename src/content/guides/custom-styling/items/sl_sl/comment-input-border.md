The border around the comment box is made of the text area's own border plus a few thin lines the widget draws around it. To change its color or corner roundness, set these CSS variables instead of styling the `textarea` directly. The variables restyle every part of the border at once, so the corners and colors always line up.

| Spremenljivka | Kaj spreminja | Privzeto |
|---|---|---|
| `--fc-input-border-color` | Barva obrobe | `#bfbfbf` |
| `--fc-input-border-color-focus` | Barva obrobe med tipkanjem | `#555` |
| `--fc-input-border-radius` | Zaobljenost zaobljenih kotov | `11px` |
| `--fc-input-border-start-start-radius` | Zaobljenost kvadratnega zgornjega levega kota (zgornjega desnega v jezikih z desno-levo usmerjenostjo) | `0` |

Add the CSS to the **Custom CSS** box on the [Widget Customization page](https://fastcomments.com/auth/my-account/customize-widget), or pass it with the `customCSS` option. You only need to set the variables you want to change.

You can also use the **Comment box border** helper right under the Custom CSS box, which writes this CSS for you.

## Spremeni barvo obrobe

[inline-code-attrs-start title = 'Barva obrobe'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## Spremeni barvo obrobe med tipkanjem

[inline-code-attrs-start title = 'Barva obrobe med tipkanjem'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## Zaobli vse štiri kote

By default the top-left corner is square. Set both radius variables to round all four corners the same:

[inline-code-attrs-start title = 'Zaobli vse štiri kote'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## Kvadratni koti

[inline-code-attrs-start title = 'Kvadratni koti'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## Uskladite z vašo blagovno znamko

[inline-code-attrs-start title = 'Barve blagovne znamke in koti'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## Različne barve v temnem načinu

When the widget is in dark mode it has the `dark` class, so you can set different values for dark mode:

[inline-code-attrs-start title = 'Barve obrobe v temnem načinu'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## Zakaj ne stilizirati besedilnega polja neposredno?

The widget draws part of the comment box border itself, around the text area. If you set `border-color` or `border-radius` only on the `textarea`, those lines keep the default style and the border looks mismatched, for example a square line running through a rounded corner. The variables above change both at once.