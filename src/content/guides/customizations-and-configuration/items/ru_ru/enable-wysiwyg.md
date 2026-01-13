[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

По умолчанию возможности форматирования в FastComments реализованы путём добавления видимых якорных тегов, таких как `<b></b>`, вокруг вашего текста. Нажатие на панель инструментов
или использование сочетаний клавиш делает это за вас. Однако некоторые сообщества могут захотеть выбрать использование форматирования без видимых якорных тегов. Это называется включением
WYSIWYG (то, что видишь — то и получаешь) редактора. Этот редактор выглядит точно так же, как и стандартный, за исключением того, что он загружает дополнительный
код, который позволяет пользователям делать текст жирным, подчёркивать и т.д. без видимых якорных тегов.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

This can also be done without code. In the widget customization page, see the "Включить расширенное форматирование" option.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; title='Enable WYSIWYG' app-screenshot-end]

---