The border around the comment box is made of the text area's own border plus a few thin lines the widget draws around it. To change its color or corner roundness, set these CSS variables instead of styling the `textarea` directly. The variables restyle every part of the border at once, so the corners and colors always line up.

| Zmienna | Co zmienia | Domyślnie |
|---|---|---|
| `--fc-input-border-color` | Kolor obramowania | `#bfbfbf` |
| `--fc-input-border-color-focus` | Kolor obramowania podczas pisania przez użytkownika | `#555` |
| `--fc-input-border-radius` | Zaokrąglenie narożników | `11px` |
| `--fc-input-border-start-start-radius` | Zaokrąglenie kwadratowego lewego górnego narożnika (prawego górnego w językach od prawej do lewej) | `0` |

Add the CSS to the **Custom CSS** box on the [Widget Customization page](https://fastcomments.com/auth/my-account/customize-widget), or pass it with the `customCSS` option. You only need to set the variables you want to change.

You can also use the **Comment box border** helper right under the Custom CSS box, which writes this CSS for you.

## Zmiana koloru obramowania

[inline-code-attrs-start title = 'Kolor obramowania'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## Zmiana koloru obramowania podczas pisania

[inline-code-attrs-start title = 'Kolor obramowania podczas pisania'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## Zaokrąglenie wszystkich czterech narożników

By default the top-left corner is square. Set both radius variables to round all four corners the same:

[inline-code-attrs-start title = 'Zaokrąglenie wszystkich czterech narożników'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## Kwadratowe narożniki

[inline-code-attrs-start title = 'Kwadratowe narożniki'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## Dopasuj do swojej marki

[inline-code-attrs-start title = 'Kolory i narożniki marki'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## Różne kolory w trybie ciemnym

When the widget is in dark mode it has the `dark` class, so you can set different values for dark mode:

[inline-code-attrs-start title = 'Kolory obramowania w trybie ciemnym'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## Dlaczego nie stylizować pola tekstowego bezpośrednio?

The widget draws part of the comment box border itself, around the text area. If you set `border-color` or `border-radius` only on the `textarea`, those lines keep the default style and the border looks mismatched, for example a square line running through a rounded corner. The variables above change both at once.

---