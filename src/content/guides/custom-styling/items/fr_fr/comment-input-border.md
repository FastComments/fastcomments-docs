La bordure autour de la boîte de commentaire est composée de la bordure propre de la zone de texte plus quelques lignes fines que le widget dessine autour. Pour changer sa couleur ou l’arrondi des coins, définissez ces variables CSS au lieu de styliser directement le `textarea`. Les variables re‑stylisent chaque partie de la bordure en même temps, de sorte que les coins et les couleurs restent alignés.

| Variable | Ce qu’elle change | Valeur par défaut |
|---|---|---|
| `--fc-input-border-color` | Couleur de la bordure | `#bfbfbf` |
| `--fc-input-border-color-focus` | Couleur de la bordure pendant que l'utilisateur tape | `#555` |
| `--fc-input-border-radius` | Arrondi des coins arrondis | `11px` |
| `--fc-input-border-start-start-radius` | Arrondi du coin supérieur gauche carré (supérieur droit dans les langues de droite à gauche) | `0` |

Ajoutez le CSS dans la zone **Custom CSS** sur la [page de personnalisation du widget](https://fastcomments.com/auth/my-account/customize-widget), ou transmettez-le avec l’option `customCSS`. Vous n’avez besoin de définir que les variables que vous souhaitez modifier.

Vous pouvez également utiliser l’assistant **Comment box border** juste sous la zone Custom CSS, qui génère ce CSS pour vous.

## Modifier la couleur de la bordure

[inline-code-attrs-start title = 'Couleur de la bordure'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## Modifier la couleur de la bordure pendant la saisie

[inline-code-attrs-start title = 'Couleur de la bordure pendant la saisie'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## Arrondir les quatre coins

Par défaut, le coin supérieur gauche est carré. Définissez les deux variables de rayon pour arrondir les quatre coins de la même façon :

[inline-code-attrs-start title = 'Arrondir les quatre coins'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## Coins carrés

[inline-code-attrs-start title = 'Coins carrés'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## Correspondre à votre marque

[inline-code-attrs-start title = 'Couleurs et coins de la marque'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## Couleurs différentes en mode sombre

Lorsque le widget est en mode sombre, il possède la classe `dark`, vous pouvez donc définir des valeurs différentes pour le mode sombre :

[inline-code-attrs-start title = 'Couleurs de bordure en mode sombre'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## Pourquoi ne pas styliser directement la zone de texte ?

Le widget dessine lui‑même une partie de la bordure de la boîte de commentaire, autour de la zone de texte. Si vous définissez `border-color` ou `border-radius` uniquement sur le `textarea`, ces lignes conservent le style par défaut et la bordure apparaît incohérente, par exemple une ligne carrée traversant un coin arrondi. Les variables ci‑dessus modifient les deux à la fois.