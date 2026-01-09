Існує два способи заборонити користувачам коментувати на вашому сайті за допомогою FastComments.

Перший — якщо ви вже знаєте їхню електронну адресу, ви можете ввести її на <a href="/auth/my-account/moderate-comments/banned-users" target="_blank">сторінці заблокованих користувачів</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

Ця сторінка доступна через Moderate Comments -> Banned Users

Коли ми йдемо блокувати користувача, ми можемо вибрати тип, або Permanent, або Permanent Shadow Ban:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

Другий спосіб заблокувати користувача — натиснути кнопку бану, яка розміщена біля кожного коментаря на сторінці Comment Moderation.

Коли ми натискаємо кнопку бану, вам буде показано кілька опцій, де можна вказати тип і тривалість бану.

### Shadow Bans

Тіньовий бан — це тип блокування, який створює враження, що коментар або голос користувача було успішно збережено, тоді як насправді це не так. Це може бути доцільним у певних ситуаціях.

### Banning Via IP Address

Якщо tenant не бажає відмовитися від цього, FastComments підтримує блокування за IP шляхом збереження хешованої версії IP-адреси коментатора.