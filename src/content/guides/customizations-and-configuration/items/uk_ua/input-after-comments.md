[related-parameter-start name = 'inputAfterComments'; type = 'boolean'; related-parameter-end]

За замовчуванням область вводу коментаря розміщена **перед** потоком коментарів. Однак, встановивши цей параметр конфігурації
в значення true, ми можемо перемістити її **після**.

[code-example-start config = {inputAfterComments: true}; linesToHighlight = [6]; title = 'Moving The Reply Box to The Bottom'; code-example-end]

Це можна налаштувати без коду, на сторінці налаштування віджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.input-after-comments'; title='Moving The Reply Box to The Bottom' app-screenshot-end]