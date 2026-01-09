[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

По умолчанию функции форматирования в FastComments реализуются добавлением видимых анкерных тегов, таких как `<b></b>`, вокруг вашего текста. Нажатие на панель инструментов или использование сочетаний клавиш делает это за вас. Однако некоторые сообщества могут захотеть включить форматирование без видимых анкерных тегов. Это называется включением редактора WYSIWYG (что видишь — то и получаешь). Этот редактор выглядит точно так же, как и стандартный, за исключением того, что он загружает дополнительный код, позволяющий пользователям выделять текст жирным, подчеркивать и т. д. без видимых анкерных тегов.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

Это также можно сделать без кода. На странице настройки виджета воспользуйтесь опцией «Включить расширенное форматирование».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; title='Enable WYSIWYG' app-screenshot-end]

---