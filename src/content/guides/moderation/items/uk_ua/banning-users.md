There are two ways to ban users from commenting on your site with FastComments.

The first is if you already know their email, you can enter it on the <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">banned users</a> page.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='Список заблокованих користувачів у розділі Модерація коментарів, з заблокованими електронними адресами та кнопкою додати нове блокування'; title='Сторінка заблокованих користувачів' app-screenshot-end]

This page can be accessed via Moderate Comments -> Banned Users

When we go to ban a user, we can pick a type, either Permanent or Permanent Shadow Ban:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='Форма нового блокування з полем електронної пошти та вибором типу блокування: Постійне або Постійне приховане блокування'; title='Блокування користувача' app-screenshot-end]

The second way to ban a user is by clicking the ban button that is placed on each comment on the Comment Moderation page.

When we click the ban button, you'll be presented with some options, where we can specify the ban type and duration.

### Email Aliases

When banning a user by email, FastComments automatically ignores `+` aliases. For example, banning `user+alias@gmail.com` will
also ban `user@gmail.com` and any other `+` variation of that address, such as `user+other@gmail.com`.

### Shadow Bans

A shadow-ban is a type of ban that makes it appear that the user's comment or vote was saved successfully, when in fact it was not. This may be desirable in certain situations.

### Banning Via IP Address

Unless a tenant wishes to opt out, FastComments supports banning via IP by storing a hashed version of the commenter's IP address.

### Searching Banned Users

Once your list grows past a page or two, you can narrow it with the search row above the table.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .filter-form'; alt='Рядок пошуку на сторінці заблокованих користувачів з випадаючим списком «Search By», випадаючим списком «Match» та полем вводу «Value»'; title='Пошук заблокованих користувачів' app-screenshot-end]

There are three controls:

- **Search By** вибирає, в якому полі шукати: Any Field, Email, Name, Banned By або Banned For Saying. Останні чотири відповідають колонкам з такими ж назвами в таблиці.
- **Match** вибирає спосіб порівняння. **Contains** знаходить ваше значення в будь‑якому місці поля, а **Equals** збігається з усім полем.
- **Value** — це текст, який треба знайти.

Every field is matched without regard to case, so searching for `SPAMMER@EXAMPLE.COM` finds a ban stored as `spammer@example.com`.

A few things worth knowing:

- **Banned For Saying** шукає текст коментаря, через який користувач був заблокований. Це спосіб знайти всіх, заблокованих за певну фразу.
- **Banned By** шукає ім'я модератора, який видав блокування, що корисно для перегляду рішень іншого модератора.
- Блокування з підстановкою зберігаються з `*`, тому пошук **Contains** за `bademail.com` знайде блокування `*@bademail.com`.
- **Name** відповідає імені, показаному в колонці Name, тому знаходить користувача, навіть якщо він змінив ім'я після блокування, і навіть якщо ви створили блокування, ввівши лише електронну адресу без імені. Ім'я, записане в блокуванні, також збігається, тому пошук за старим або поточним іменем працює.
- **Any Field** шукає одночасно в електронній пошті, імені, модераторі, який заблокував, та тексті заблокованого коментаря.

Your search is part of the page URL, so you can share a filtered list with other moderators the same way you share other moderation links. Paging through results keeps the search applied, starting a new search returns you to the first page, and **Clear** returns to the full list.