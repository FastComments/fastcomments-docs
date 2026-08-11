---
[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

По умолчанию функции форматирования в FastComments реализуются добавлением видимых тегов‑якорей, таких как `<b></b>`, вокруг вашего текста. Нажатие на панель инструментов
или использование сочетаний клавиш делает это за вас. Однако некоторые сообщества могут захотеть использовать форматирование без тегов‑якорей. Это называется включением
WYSIWYG (what you see is what you get) редактора. Этот редактор выглядит точно так же, как и редактор по умолчанию, за исключением того, что он загружает дополнительный
код, позволяющий пользователям делать полужирный, подчёркнутый и т.д. текст без видимых тегов‑якорей.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Включение редактирования WYSIWYG'; code-example-end]

Это также можно сделать без кода. На странице настройки виджета смотрите опцию "Enable Advanced Formatting".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; alt='Страница настройки виджета с отмеченным флажком Enable Advanced Formatting для включения редактора WYSIWYG'; title='Включить WYSIWYG' app-screenshot-end]

---