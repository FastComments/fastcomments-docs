---
[related-parameter-start name = 'noNewRootComments'; type = 'boolean'; related-parameter-end]

Встановлення `noNewRootComments` в `true` призведе до того, що віджет приховає область відповіді на кореневі коментарі, але все ще дозволятиме користувачам відповідати
на дочірні коментарі. Ви, наприклад, можете встановити це умовно під час завантаження сторінки, щоб лише деяким користувачам дозволити залишати коментарі верхнього рівня.

[code-example-start config = {noNewRootComments: true}; linesToHighlight = [6]; title = 'Prevent New Root Comments'; code-example-end]

---