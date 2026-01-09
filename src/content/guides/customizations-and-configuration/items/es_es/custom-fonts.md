[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments está diseñado para ser personalizado, y la fuente que usan nuestros widgets no es una excepción.

Por defecto, FastComments usa el `system font stack` para verse lo mejor posible en una amplia gama de dispositivos.

Para definir tus propias fuentes, consulta la [documentación de CSS personalizado](/guide-customizations-and-configuration.html#custom-css).

Allí encontrarás una forma de definir CSS personalizado, lo que te permitirá establecer las fuentes que desees.

#### Cómo definir la fuente

Para sobrescribir la fuente, recomendamos que definas tu CSS usando los selectores `.fast-comments, textarea`. Por ejemplo:

[inline-code-attrs-start title = 'Ejemplo de fuente externa personalizada'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap');
.fast-comments, textarea {
    font-family: 'Roboto', sans-serif;
}
[inline-code-end]

---