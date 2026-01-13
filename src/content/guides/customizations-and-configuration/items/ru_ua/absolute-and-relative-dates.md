[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

По умолчанию используются локализованные относительные даты. Например, рядом с недавно оставленным комментарием вы можете увидеть "11 минут назад".

Иногда требуется или желательно сохранить этот формат относительной даты, но также отображать полную дату рядом с ним; в этом случае установите этот параметр в true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

Это можно настроить без кода, на странице настройки виджета, в разделе «Дополнительные параметры». Сначала вам нужно будет включить «Абсолютные даты», чтобы увидеть эту опцию в интерфейсе.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; title='Use Both Absolute and Relative Dates' app-screenshot-end]