[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments je dizajniran da bude prilagodljiv, i font koji koriste naši widgeti nije izuzetak.

Podrazumevano, FastComments koristi `system font stack` da izgleda što bolje na širokom spektru uređaja.

Da biste definisali sopstvene fontove, pogledajte [Custom CSS dokumentaciju](/guide-customizations-and-configuration.html#custom-css).

Tamo ćete pronaći način da definišete prilagođeni CSS, što će vam omogućiti da postavite željene fontove.

#### Kako definisati font

Da biste prebrisali font, preporučujemo da definišete svoj CSS koristeći selektore `.fast-comments, textarea`. Na primer:

[inline-code-attrs-start title = 'Primer prilagođenog spoljnog fonta'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap');
.fast-comments, textarea {
    font-family: 'Roboto', sans-serif;
}
[inline-code-end]

---