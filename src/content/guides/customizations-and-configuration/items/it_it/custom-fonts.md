[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments è progettato per essere personalizzato, e il font utilizzato dai nostri widget non fa eccezione.

Per impostazione predefinita, FastComments utilizza la `system font stack` per apparire al meglio su un'ampia gamma di dispositivi.

Per definire i tuoi font, vedi la [documentazione sul CSS personalizzato](/guide-customizations-and-configuration.html#custom-css).

Lì troverai un modo per definire CSS personalizzato, che ti permetterà di impostare i font desiderati.

#### Come definire il font

Per sovrascrivere il font, ti consigliamo di definire il tuo CSS usando i selettori `.fast-comments, textarea`. Per esempio:

[inline-code-attrs-start title = 'Esempio di font esterno personalizzato'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap');
.fast-comments, textarea {
    font-family: 'Roboto', sans-serif;
}
[inline-code-end]