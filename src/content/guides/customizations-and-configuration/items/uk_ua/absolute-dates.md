[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

За замовчуванням використовуються локалізовані відносні дати. Наприклад, поруч із нещодавно залишеним коментарем ви можете бачити "11 хвилин тому".

Можливо, буде необхідно або бажано використовувати абсолютні дати, у цьому випадку ви встановлюєте цей параметр у true. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Use Absolute Dates'; code-example-end]

Це можна налаштувати без коду на сторінці налаштування віджета в розділі Розширені параметри:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; alt='Розширені параметри на сторінці налаштування віджета з увімкненим перемикачем абсолютних дат'; title='Використання абсолютних дат' app-screenshot-end]

---