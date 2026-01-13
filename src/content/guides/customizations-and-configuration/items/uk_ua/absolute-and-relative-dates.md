[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

За замовчуванням використовуються локалізовані відносні дати. Наприклад, поруч із нещодавно залишеним коментарем ви можете побачити "11 хвилин тому".

Іноді може виникнути необхідність або бажання зберегти цей відносний формат дати, але також показувати повну дату поруч із ним; у цьому випадку встановіть цей параметр у true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

Це можна налаштувати без коду на сторінці налаштування віджета, у розділі Розширені параметри. Спочатку вам потрібно ввімкнути Absolute Dates, щоб побачити цю опцію в інтерфейсі.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; title='Use Both Absolute and Relative Dates' app-screenshot-end]