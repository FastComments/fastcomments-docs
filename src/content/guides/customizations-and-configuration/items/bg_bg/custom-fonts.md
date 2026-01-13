[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments е предназначен да бъде персонализиран, а шрифтът, който използват нашите уиджети, не е изключение.

По подразбиране FastComments използва `system font stack`, за да изглежда възможно най-добре на широка гама от устройства.

За да дефинирате свои собствени шрифтове, вижте [Документация за персонализиран CSS](/guide-customizations-and-configuration.html#custom-css).

Там ще намерите начин за дефиниране на персонализиран CSS, който ще ви позволи да зададете желаните шрифтове.

#### Как да зададете шрифта

За да презапишете шрифта, препоръчваме да дефинирате своя CSS, използвайки селекторите `.fast-comments, textarea`. Например:

[inline-code-attrs-start title = 'Пример за персонализиран външен шрифт'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap');
.fast-comments, textarea {
    font-family: 'Roboto', sans-serif;
}
[inline-code-end]

---