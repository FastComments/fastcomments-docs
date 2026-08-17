[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Коли користувач коментує за допомогою FastComments вперше, ми спробуємо отримати його аватар з <a href="https://gravatar.com/" target="_blank">https://gravatar.com/</a>.

Однак, якщо аватар не знайдено або користувач ніколи не встановлює його у своєму обліковому записі, ми відображаємо статичне зображення аватара за замовчуванням.

Щоб вказати власне статичне зображення аватара, можна використати налаштування *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Перевизначити аватар за замовчуванням'; code-example-end]

Це також можна зробити без коду. На сторінці налаштування віджета перегляньте розділ «Аватар за замовчуванням».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; alt='Розділ «Аватар за замовчуванням» на сторінці налаштування віджета, де ви вказуєте URL резервного зображення аватара'; title='Налаштування аватара за замовчуванням' app-screenshot-end]

Зверніть увагу, що визначення аватара для конкретного користувача, наприклад за допомогою SSO, розглядається в окремому розділі.