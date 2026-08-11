[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Для автентифікації FastComments залежить від увімкнених у вашому браузері сторонніх файлів cookie. Без них користувачі завжди будуть змушені залишати свою електронну пошту, щоб залишити коментар (якщо поле вводу електронної пошти не приховано), і їхні коментарі завжди будуть позначені як неперевірені (за замовчуванням).

Щоб обійти це, ви можете ввімкнути обход сторонніх файлів cookie. 

Коли це налаштування ввімкнено, воно викликає невелике спливаюче вікно, яке показує повідомлення про те, що користувач входить у систему. Це спливаюче вікно з’являється щоразу, коли користувач взаємодіє з віджетом коментарів; наприклад, коли залишає коментар.

Ми можемо зробити це в коді, встановивши прапорець **enableThirdPartyCookieBypass** у значення true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Увімкнення обходу сторонніх файлів cookie'; code-example-end]

Ми також можемо налаштувати це через інтерфейс налаштування віджету, у розділі `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; alt='Сторінка налаштування віджету з позначеним прапорцем Увімкнути спливаюче вікно сторонніх файлів cookie'; title='Увімкнення обходу сторонніх файлів cookie' app-screenshot-end]