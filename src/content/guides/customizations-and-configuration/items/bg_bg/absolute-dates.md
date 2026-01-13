[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

По подразбиране се използват локализирани относителни дати. Например, до наскоро публикуван коментар може да видите "преди 11 минути".

Може да е необходимо или желателно да използвате абсолютни дати, в този случай задайте този параметър на true. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Use Absolute Dates'; code-example-end]

Това може да се персонализира без писане на код на страницата за персонализиране на уиджета, в Разширени опции:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; title='Use Absolute Dates' app-screenshot-end]