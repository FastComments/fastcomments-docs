[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Для автентифікації FastComments залежить від того, щоб сторонні cookie були увімкнені у вашому браузері. Без них користувачам завжди доведеться залишати свою електронну пошту, щоб коментувати (якщо тільки поле вводу електронної пошти не приховане), і їхні коментарі за замовчуванням завжди відображатимуться як неперевірені.

Щоб обійти це, ви можете увімкнути обхід сторонніх cookie.

Коли цей параметр увімкнено, з’являтиметься невелике спливаюче вікно з повідомленням, що користувач входить у систему. Це спливаюче вікно показується щоразу, коли користувач взаємодіє з віджетом коментарів; наприклад, коли він залишає коментар.

Ми можемо зробити це в коді, встановивши прапорець **enableThirdPartyCookieBypass** у true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Enabling Third-Party Cookie Bypass'; code-example-end]

Ми також можемо налаштувати це через інтерфейс налаштування віджета, у розділі `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; title='Enabling Third-Party Cookie Bypass' app-screenshot-end]

---