[related-parameter-start name = 'noStyles'; type = 'boolean'; related-parameter-end]

Для более крупных проектов пользовательского стилирования может быть желательно начать с чистого листа и полностью отказаться от использования стилей по умолчанию.

All default styling can be removed by setting the **noStyles** parameter to true, as follows:

[code-example-start config = {noStyles: true}; linesToHighlight = [6]; title = 'Disabling All Default Styles'; code-example-end]

Это можно настроить без кода на странице настройки виджета в разделе Advanced Options:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.disable-all-default-styling']; selector = '.disable-all-default-styling'; alt='Флажок отключения всех стилей по умолчанию, включенный в разделе «Дополнительные параметры» на странице настройки виджета'; title='Отключение всех стилей по умолчанию' app-screenshot-end]