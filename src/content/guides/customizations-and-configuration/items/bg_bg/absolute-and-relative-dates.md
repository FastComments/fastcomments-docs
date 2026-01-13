[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

По подразбиране се използват локализирани относителни дати. Например, до наскоро оставен коментар може да видите "преди 11 минути".

Може да е необходимо или желателно да запазите този относителен формат на датата, но също така да покажете пълната дата заедно с него; в този случай задайте този параметър на true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

Това може да бъде персонализирано без код, на страницата за персонализиране на джаджата, в секцията Разширени опции. Първо ще трябва да активирате Absolute Dates, за да видите тази опция в потребителския интерфейс.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; title='Use Both Absolute and Relative Dates' app-screenshot-end]