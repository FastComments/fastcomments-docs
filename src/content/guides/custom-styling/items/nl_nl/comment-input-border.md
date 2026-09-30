De rand rond het reactie‑vak bestaat uit de eigen rand van het tekstgebied plus een paar dunne lijnen die de widget eromheen tekent. Om de kleur of de afronding van de hoeken te wijzigen, stel je deze CSS‑variabelen in plaats van het `textarea` direct te stylen. De variabelen stylen elk deel van de rand in één keer opnieuw, zodat de hoeken en kleuren altijd op elkaar afgestemd zijn.

| Variabele | Wat het verandert | Standaard |
|---|---|---|
| `--fc-input-border-color` | Randkleur | `#bfbfbf` |
| `--fc-input-border-color-focus` | Randkleur terwijl de gebruiker typt | `#555` |
| `--fc-input-border-radius` | Afronding van de afgeronde hoeken | `11px` |
| `--fc-input-border-start-start-radius` | Afronding van de vierkante linkerbovenhoek (rechterbovenhoek in rechts‑naar‑links talen) | `0` |

Voeg de CSS toe aan het **Custom CSS**‑vak op de [Widget Customization page](https://fastcomments.com/auth/my-account/customize-widget), of geef het door met de `customCSS`‑optie. Je hoeft alleen de variabelen in te stellen die je wilt wijzigen.

Je kunt ook de **Comment box border**‑helper direct onder het Custom CSS‑vak gebruiken, die deze CSS voor je genereert.

## Randkleur wijzigen

[inline-code-attrs-start title = 'Randkleur'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## Randkleur wijzigen tijdens typen

[inline-code-attrs-start title = 'Randkleur tijdens typen'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## Alle vier hoeken afronden

Standaard is de linkerbovenhoek vierkant. Stel beide radius‑variabelen in om alle vier hoeken evenredig af te ronden:

[inline-code-attrs-start title = 'Alle vier hoeken afronden'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## Vierkante hoeken

[inline-code-attrs-start title = 'Vierkante hoeken'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## Pas aan op je merk

[inline-code-attrs-start title = 'Merkkleuren en hoeken'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## Verschillende kleuren in donkere modus

Wanneer de widget in donkere modus staat, heeft hij de `dark`‑klasse, zodat je verschillende waarden voor donkere modus kunt instellen:

[inline-code-attrs-start title = 'Randkleuren in donkere modus'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## Waarom de tekstvak niet direct stylen?

De widget tekent zelf een deel van de rand van het reactie‑vak, rond het tekstgebied. Als je alleen `border-color` of `border-radius` op het `textarea` instelt, behouden die lijnen de standaardstijl en ziet de rand er niet bij elkaar passend uit, bijvoorbeeld een vierkante lijn die door een afgeronde hoek loopt. De bovenstaande variabelen wijzigen beide tegelijk.