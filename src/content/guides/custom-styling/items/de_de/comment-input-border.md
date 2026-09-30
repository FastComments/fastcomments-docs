Der Rand um das Kommentarfeld besteht aus dem eigenen Rand des Textbereichs plus einigen dünnen Linien, die das Widget darum zeichnet. Um seine Farbe oder Rundung der Ecken zu ändern, setzen Sie diese CSS-Variablen anstatt das `textarea` direkt zu stylen. Die Variablen gestalten den gesamten Rand auf einmal neu, sodass die Ecken und Farben immer übereinstimmen.

| Variable | Was es ändert | Standard |
|---|---|---|
| `--fc-input-border-color` | Randfarbe | `#bfbfbf` |
| `--fc-input-border-color-focus` | Randfarbe, während der Benutzer tippt | `#555` |
| `--fc-input-border-radius` | Rundheit der abgerundeten Ecken | `11px` |
| `--fc-input-border-start-start-radius` | Rundheit der quadratischen oberen linken Ecke (oben rechts in Rechts-nach-Links-Sprachen) | `0` |

Fügen Sie das CSS in das **Custom CSS**-Feld auf der [Widget Customization page](https://fastcomments.com/auth/my-account/customize-widget) hinzu oder übergeben Sie es mit der Option `customCSS`. Sie müssen nur die Variablen setzen, die Sie ändern möchten.

Sie können auch den **Comment box border**-Assistenten direkt unter dem Custom CSS-Feld verwenden, der dieses CSS für Sie erstellt.

## Randfarbe ändern

[inline-code-attrs-start title = 'Randfarbe'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## Randfarbe beim Tippen ändern

[inline-code-attrs-start title = 'Randfarbe beim Tippen'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## Alle vier Ecken abrunden

Standardmäßig ist die obere linke Ecke quadratisch. Setzen Sie beide Radius-Variablen, um alle vier Ecken gleich abzurunden:

[inline-code-attrs-start title = 'Alle vier Ecken abrunden'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## Quadratische Ecken

[inline-code-attrs-start title = 'Quadratische Ecken'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## An Ihre Marke anpassen

[inline-code-attrs-start title = 'Markenfarben und Ecken'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## Unterschiedliche Farben im Dark Mode

Wenn das Widget im Dark Mode ist, hat es die Klasse `dark`, sodass Sie für den Dark Mode andere Werte setzen können:

[inline-code-attrs-start title = 'Randfarben im Dark Mode'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## Warum das Textfeld nicht direkt stylen?

Das Widget zeichnet einen Teil des Kommentarfeld‑Randes selbst, um den Textbereich herum. Wenn Sie `border-color` oder `border-radius` nur am `textarea` setzen, behalten diese Linien den Standardstil bei und der Rand wirkt nicht übereinstimmend, zum Beispiel eine quadratische Linie, die durch eine abgerundete Ecke verläuft. Die oben genannten Variablen ändern beides gleichzeitig.

---