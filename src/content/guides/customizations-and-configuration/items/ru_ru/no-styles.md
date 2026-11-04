[related-parameter-start name = 'noStyles'; type = 'boolean'; related-parameter-end]

Для более масштабных проектов кастомизации стилей может быть желательно начать с чистого листа и вообще не использовать стили по умолчанию.

Все стили по умолчанию можно убрать, установив параметр **noStyles** в значение true, следующим образом:

[code-example-start config = {noStyles: true}; linesToHighlight = [6]; title = 'Отключение всех стилей по умолчанию'; code-example-end]

Это можно настроить без кода, на странице настройки виджета, в разделе Advanced Options:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.disable-all-default-styling']; selector = '.disable-all-default-styling'; alt='Чекбокс отключения всех стилей по умолчанию включён в разделе Advanced Options на странице настройки виджета'; title='Отключение всех стилей по умолчанию' app-screenshot-end]