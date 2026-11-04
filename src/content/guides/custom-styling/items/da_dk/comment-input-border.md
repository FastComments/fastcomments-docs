Kanten omkring kommentarboksen består af tekstområdets egen kant plus et par tynde linjer, som widgeten tegner omkring den. For at ændre dens farve eller hjørnerundhed skal du angive disse CSS-variabler i stedet for at style `textarea` direkte. Variablerne restyler alle dele af kanten på én gang, så hjørner og farver altid passer sammen.

| Variabel | Hvad den ændrer | Standard |
|---|---|---|
| `--fc-input-border-color` | Kantfarve | `#bfbfbf` |
| `--fc-input-border-color-focus` | Kantfarve mens brugeren skriver | `#555` |
| `--fc-input-border-radius` | Rundhed af de afrundede hjørner | `11px` |
| `--fc-input-border-start-start-radius` | Rundhed af det firkantede hjørne øverst til venstre (øverst til højre i højre-til-venstre-sprog) | `0` |

Tilføj CSS'en i boksen **Custom CSS** på [Widget Customization-siden](https://fastcomments.com/auth/my-account/customize-widget), eller send den med `customCSS`-indstillingen. Du behøver kun at angive de variabler, du vil ændre.

Du kan også bruge hjælpefunktionen **Comment box border** lige under Custom CSS-boksen, som skriver denne CSS for dig.

## Ændr kantfarven

[inline-code-attrs-start title = 'Kantfarve'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## Ændr kantfarven mens der skrives

[inline-code-attrs-start title = 'Kantfarve mens der skrives'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## Afrund alle fire hjørner

Som standard er hjørnet øverst til venstre firkantet. Angiv begge radius-variabler for at afrunde alle fire hjørner ens:

[inline-code-attrs-start title = 'Afrund alle fire hjørner'; type = 'css'; isFunctional = false; inline-code-attrs-end]
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

## Match dit brand

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

Når widgeten er i mørk tilstand, har den `dark`-klassen, så du kan angive forskellige værdier for mørk tilstand:

[inline-code-attrs-start title = 'Kantfarver i mørk tilstand'; type = 'css'; isFunctional = false; inline-code-attrs-end]
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

Widgeten tegner selv en del af kommentarboksens kant, omkring tekstområdet. Hvis du kun angiver `border-color` eller `border-radius` på `textarea`, beholder disse linjer standardstilen, og kanten ser upassende ud, for eksempel en firkantet linje, der løber gennem et afrundet hjørne. Variablerne ovenfor ændrer begge på én gang.