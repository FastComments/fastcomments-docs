Існує два способи заборонити користувачам коментувати ваш сайт за допомогою FastComments.

Перший — якщо ви вже знаєте їхню електронну пошту, ви можете ввести її на сторінці <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">banned users</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

Цю сторінку можна відкрити через Модерація коментарів -> Заблоковані користувачі

Коли ми збираємося заблокувати користувача, ми можемо вибрати тип — Permanent або Permanent Shadow Ban:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

Другий спосіб заблокувати користувача — натиснути кнопку блокування, яка розміщена біля кожного коментаря на сторінці модерації коментарів.

Коли натискаєте кнопку блокування, вам будуть представлені опції, де можна вказати тип блокування та тривалість.

### Email Aliases

При блокуванні користувача за електронною поштою FastComments автоматично ігнорує `+` аліаси. Наприклад, блокування `user+alias@gmail.com` також заблокує `user@gmail.com` та будь-яку іншу `+` варіацію цієї адреси, наприклад `user+other@gmail.com`.

### Shadow Bans

Тіньове блокування — це тип блокування, при якому створюється враження, що коментар або голос користувача було успішно збережено, хоча насправді це не так. Це може бути корисним у певних ситуаціях.

### Banning Via IP Address

Якщо тенант не бажає відмовитися, FastComments підтримує блокування за IP, зберігаючи хешовану версію IP-адреси коментатора.

---