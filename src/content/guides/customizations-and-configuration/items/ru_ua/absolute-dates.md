[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

По умолчанию используются локализованные относительные даты. Например, рядом с недавно оставленным комментарием вы можете увидеть "11 минут назад".

Иногда может потребоваться или быть желательным использование абсолютных дат, в этом случае установите этот параметр в true. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Use Absolute Dates'; code-example-end]

Это можно настроить без кода на странице настройки виджета, в разделе «Дополнительные параметры»:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; title='Use Absolute Dates' app-screenshot-end]

---