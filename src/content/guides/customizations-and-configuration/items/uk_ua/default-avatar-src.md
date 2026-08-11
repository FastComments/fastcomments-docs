[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Коли користувач коментує за допомогою FastComments вперше, ми спробуємо отримати його аватарку з <a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a>.

Однак, якщо аватарку не знайдено або користувач ніколи не встановлює її у своєму обліковому записі, ми відображаємо статичне зображення типового аватара.

Щоб вказати власне статичне зображення аватара, можна використати налаштування *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Перевизначити типову аватарку'; code-example-end]

Це також можна зробити без коду. На сторінці налаштування віджета перегляньте розділ «Default Avatar».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; alt='Розділ «Типова аватарка» на сторінці налаштування віджета, де ви встановлюєте URL резервного зображення аватарки'; title='Налаштування типової аватарки' app-screenshot-end]

Зверніть увагу, що визначення аватара для конкретного користувача, наприклад за допомогою SSO, розглядається в окремому розділі.