La bordatura intorno alla casella dei commenti è composta dal bordo dell'area di testo stessa più alcune linee sottili che il widget disegna attorno. Per cambiare il colore o la rotondità degli angoli, imposta queste variabili CSS invece di stilizzare direttamente il `textarea`. Le variabili ridisegnano ogni parte del bordo in una volta, così gli angoli e i colori sono sempre allineati.

| Variabile | Cosa cambia | Predefinito |
|---|---|---|
| `--fc-input-border-color` | Colore del bordo | `#bfbfbf` |
| `--fc-input-border-color-focus` | Colore del bordo mentre l'utente sta digitando | `#555` |
| `--fc-input-border-radius` | Rotondità degli angoli arrotondati | `11px` |
| `--fc-input-border-start-start-radius` | Rotondità dell'angolo in alto a sinistra quadrato (in alto a destra nelle lingue da destra a sinistra) | `0` |

Aggiungi il CSS nella casella **Custom CSS** nella [pagina di personalizzazione del widget](https://fastcomments.com/auth/my-account/customize-widget), oppure passalo con l'opzione `customCSS`. È necessario impostare solo le variabili che desideri modificare.

Puoi anche utilizzare l'assistente **Comment box border** subito sotto la casella Custom CSS, che genera questo CSS per te.

## Cambia il colore del bordo

[inline-code-attrs-start title = 'Colore del bordo'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## Cambia il colore del bordo durante la digitazione

[inline-code-attrs-start title = 'Colore del bordo durante la digitazione'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## Arrotonda tutti e quattro gli angoli

Di default l'angolo in alto a sinistra è quadrato. Imposta entrambe le variabili di raggio per arrotondare tutti e quattro gli angoli allo stesso modo:

[inline-code-attrs-start title = 'Arrotonda tutti e quattro gli angoli'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## Angoli quadrati

[inline-code-attrs-start title = 'Angoli quadrati'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## Abbina al tuo brand

[inline-code-attrs-start title = 'Colori e angoli del brand'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## Colori diversi in modalità scura

Quando il widget è in modalità scura ha la classe `dark`, quindi puoi impostare valori diversi per la modalità scura:

[inline-code-attrs-start title = 'Colori del bordo in modalità scura'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## Perché non stilizzare direttamente l'area di testo?

Il widget disegna parte del bordo della casella dei commenti da solo, attorno all'area di testo. Se imposti `border-color` o `border-radius` solo sul `textarea`, quelle linee mantengono lo stile predefinito e il bordo appare non corrispondente, ad esempio una linea quadrata che attraversa un angolo arrotondato. Le variabili sopra cambiano entrambi contemporaneamente.