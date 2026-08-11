---
[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

По умолчанию используются локализованные относительные даты. Например, рядом с недавно оставленным комментарием вы можете увидеть "11 минут назад".

Может потребоваться или быть желательным использовать абсолютные даты, в этом случае вы устанавливаете этот параметр в true. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Использовать абсолютные даты'; code-example-end]

Это можно настроить без кода, на странице настройки виджета, в разделе Advanced Options:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; alt='Advanced Options на странице настройки виджета с включённым переключателем абсолютных дат'; title='Использовать абсолютные даты' app-screenshot-end]

---