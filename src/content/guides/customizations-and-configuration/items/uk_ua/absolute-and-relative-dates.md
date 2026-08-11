[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

За замовчуванні використовуються локалізовані відносні дати. Наприклад, поруч із нещодавно залишеним коментарем ви можете бачити "11 хвилин тому".

Можливо, буде необхідно або бажано залишити цей відносний формат дати, але також показати повну дату поруч, у цьому випадку ви встановлюєте цей параметр у true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Використовувати як абсолютні, так і відносні дати'; code-example-end]

Це можна налаштувати без коду на сторінці налаштування віджету в розділі Додаткові параметри. Спочатку вам потрібно ввімкнути Абсолютні дати, щоб побачити цю опцію в інтерфейсі.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; alt='Додаткові параметри на сторінці налаштування віджету з увімкненими як абсолютними датами, так і комбінованим налаштуванням відносної дати'; title='Використовувати як абсолютні, так і відносні дати' app-screenshot-end]

---