[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

По подразбиране се използват локализирани относителни дати. Например, до наскоро оставен коментар може да видите "11 minutes ago".

Може да е необходимо или желателно да се използват абсолютни дати, в такъв случай задавате този параметър на true. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Use Absolute Dates'; code-example-end]

Това може да се персонализира без код, на страницата за персонализиране на уиджета, под Advanced Options:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; alt='Разширени опции на страницата за персонализиране на уиджета с включен превключвател за абсолютни дати'; title='Use Absolute Dates' app-screenshot-end]