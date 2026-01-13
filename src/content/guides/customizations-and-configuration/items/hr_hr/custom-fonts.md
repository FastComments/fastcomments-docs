[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments je dizajniran da se prilagodi, a font koji koriste naši widgeti nije iznimka.

Prema zadanim postavkama, FastComments koristi `system font stack` kako bi izgledao što bolje na širokom rasponu uređaja.

Za definiranje vlastitih fontova, pogledajte [Dokumentacija o prilagođenom CSS-u](/guide-customizations-and-configuration.html#custom-css).

Tamo ćete pronaći način za definiranje prilagođenog CSS-a, što će vam omogućiti definiranje željenih fontova.

#### Kako definirati font

Da biste nadjačali font, preporučujemo da svoj CSS definirate koristeći selektore `.fast-comments, textarea`. Na primjer:

[inline-code-attrs-start title = 'Primjer prilagođenog vanjskog fonta'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap');
.fast-comments, textarea {
    font-family: 'Roboto', sans-serif;
}
[inline-code-end]

---