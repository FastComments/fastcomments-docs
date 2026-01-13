[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

За замовчуванням використовуються локалізовані відносні дати. Наприклад, поряд із нещодавно залишеним коментарем ви можете побачити "11 хвилин тому".

Іноді може бути необхідно або бажано використовувати абсолютні дати, у такому випадку цей параметр потрібно встановити в true. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Use Absolute Dates'; code-example-end]

Це можна налаштувати без коду на сторінці налаштування віджета, в розділі Додаткові параметри:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; title='Use Absolute Dates' app-screenshot-end]

---