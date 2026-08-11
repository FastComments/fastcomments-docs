[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

По умолчанию используются локализованные относительные даты. Например, рядом с недавно оставленным комментарием вы можете увидеть «11 минут назад».

Возможно, потребуется или захотите сохранить этот относительный формат даты, но также отображать полную дату рядом с ним, в этом случае вы устанавливаете этот параметр в true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

Это можно настроить без кода на странице настройки виджета в разделе **Advanced Options**. Сначала вам нужно включить **Absolute Dates**, чтобы увидеть эту опцию в пользовательском интерфейсе.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; alt='Advanced Options на странице настройки виджета с включёнными как абсолютными датами, так и объединённой настройкой относительной даты'; title='Использовать как абсолютные, так и относительные даты' app-screenshot-end]