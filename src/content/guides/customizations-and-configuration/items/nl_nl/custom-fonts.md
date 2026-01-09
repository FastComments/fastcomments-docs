[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments is ontworpen om aangepast te worden, en het lettertype dat onze widgets gebruiken is daarop geen uitzondering.

Standaard gebruikt FastComments de `system font stack` zodat het er op een breed scala aan apparaten zo goed mogelijk uitziet.

Om je eigen lettertypen te definiëren, zie de [Documentatie voor Aangepaste CSS](/guide-customizations-and-configuration.html#custom-css).

Daar vind je een manier om aangepaste CSS te definiëren, waarmee je de gewenste lettertypen kunt instellen.

#### Hoe het lettertype te definiëren

Om het lettertype te overschrijven raden we aan je CSS te definiëren met de `.fast-comments, textarea` selectors. Bijvoorbeeld:

[inline-code-attrs-start title = 'Voorbeeld van aangepast extern lettertype'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap');
.fast-comments, textarea {
    font-family: 'Roboto', sans-serif;
}
[inline-code-end]

---