[related-parameter-start name = 'noStyles'; type = 'boolean'; related-parameter-end]

В больших проектах по индивидуальной стилизации может быть целесообразно начать с чистого листа и вовсе не использовать стили по умолчанию.

Вся стилизация по умолчанию может быть удалена, установив параметр **noStyles** в true, как показано ниже:

[code-example-start config = {noStyles: true}; linesToHighlight = [6]; title = 'Disabling All Default Styles'; code-example-end]

Это можно настроить без кода на странице настройки виджета, в разделе «Дополнительные параметры»:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.disable-all-default-styling']; selector = '.disable-all-default-styling'; title='Disabling All Default Styles' app-screenshot-end]