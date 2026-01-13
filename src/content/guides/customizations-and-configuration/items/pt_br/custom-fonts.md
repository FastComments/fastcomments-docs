[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

O FastComments foi projetado para ser personalizado, e a fonte que nossos widgets usam não é exceção.

Por padrão, o FastComments usa o `system font stack` para ter a melhor aparência possível em uma ampla variedade de dispositivos.

Para definir suas próprias fontes, consulte a [documentação de CSS personalizado](/guide-customizations-and-configuration.html#custom-css).

Lá você encontrará uma forma de definir CSS personalizado, o que permitirá especificar as fontes desejadas.

#### Como Definir a Fonte

Para sobrescrever a fonte, recomendamos definir seu CSS usando os seletores `.fast-comments, textarea`. Por exemplo:

[inline-code-attrs-start title = 'Exemplo de Fonte Externa Personalizada'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap');
.fast-comments, textarea {
    font-family: 'Roboto', sans-serif;
}
[inline-code-end]

---