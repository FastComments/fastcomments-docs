[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

По подразбиране се използват локализирани относителни дати. Например, до наскоро оставен коментар може да видите "11 minutes ago".

Може да е необходимо или желателно да запазите този относителен формат на датата, но също така да покажете пълната дата до него, в който случай задавате този параметър на true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Използване и на абсолютни и на относителни дати'; code-example-end]

Това може да се персонализира без код, на страницата за персонализиране на уиджета, под Разширени опции. Първо ще трябва да активирате Абсолютни дати, за да видите тази опция в потребителския интерфейс.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; alt='Разширени опции на страницата за персонализиране на уиджета с активирани както абсолютни дати, така и комбинираната настройка за относителна дата'; title='Използване и на абсолютни и на относителни дати' app-screenshot-end]