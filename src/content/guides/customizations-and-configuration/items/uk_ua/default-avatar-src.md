[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Коли користувач коментує через FastComments вперше, ми спробуємо отримати його аватар з <a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a>.

Однак якщо ми не знайдемо аватар або користувач ніколи не встановлював його у своєму обліковому записі, ми відображаємо статичне зображення аватара за замовчуванням.

Щоб вказати власне статичне зображення аватара, можна використати налаштування *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

Це також можна зробити без коду. На сторінці налаштування віджета див. розділ «Аватар за замовчуванням».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; title='Customizing The Default Avatar' app-screenshot-end]

Зверніть увагу, що визначення аватара для конкретного користувача, наприклад при SSO, описано в окремому розділі.