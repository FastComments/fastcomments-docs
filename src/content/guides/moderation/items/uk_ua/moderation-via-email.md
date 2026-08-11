FastComments підтримує щоденний, щотижневий або щомісячний електронний дайджест для модераторів та адміністраторів.

Частоту можна налаштувати <a href="" target="_blank">тут</a>.

[app-screenshot-start url='/auth/my-account/edit-notifications?demoDigestFrequencyValue=0'; linkUrl='/auth/my-account/edit-notifications'; selector = '.content form'; alt='Форма редагування сповіщень, де налаштовано отримання дайджесту щодня, щотижня або щомісяця'; title='Налаштування частоти дайджесту' app-screenshot-end]

Крім загальної статистики щодо ваших коментарів, він також перелічить три найновіші коментарі, які потребують перегляду.

Для кожного з цих коментарів надаються прямі магічні посилання, щоб:
- Підтвердити коментар.
- Позначити коментар як переглянутий і перейти на сторінку відповіді.
- Позначити коментар як спам.

Ці посилання для кожного коментаря автоматично автентифікуватимуть вас і виконуватимуть дію з вашого електронного листа.

Крім того, у дайджесті розташована кнопка «Модерувати коментарі», яка виконає ту ж автентифікацію і перенесе вас на сторінку модерації коментарів.

Зверніть увагу, що ці магічні посилання закінчують термін дії через певний час.

[app-screenshot-start url='/test-e2e/email/tenant-comment-digest?HOST=http%3A%2F%2Flocalhost%3A3001&stats=%7B"hasHistory"%3Atrue%2C"newCommentsCount"%3A10002%2C"hasNewCommentsIncreased"%3Atrue%2C"hasNewCommentsDecreased"%3Afalse%2C"approvedCommentsCount"%3A44%2C"hasApprovedCommentsIncreased"%3Afalse%2C"hasApprovedCommentsDecreased"%3Atrue%2C"spamCommentsCount"%3A21%2C"hasSpamCommentsIncreased"%3Afalse%2C"hasSpamCommentsDecreased"%3Atrue%2C"newUsersCount"%3A30%2C"hasNewUsersIncreased"%3Atrue%2C"hasNewUsersFalse"%3Afalse%7D&BANNER_TEXT=FastComments%20Monthly%20Digest&commentCount=100000&hasCommentsNeedsReview=true&comments=%5B%7B"commenterName"%3A"Devon%20Winrick"%2C"commentHTML"%3A"This%20is%20a%20very%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Devon"%2C"commentHTML"%3A"This%20is%20a%20somewhat%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Bob"%2C"commentHTML"%3A"This%20is%20a%20kind%20of%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Ffastcomments.com%2Fimages%2Funknown-person.png"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%5D&locale=en_us&digestEmail=%7B"tenantId"%3A"tenant-id"%2C"userId"%3A"user-id"%2C"_id"%3A"some-id"%2C"temporaryId"%3A"temporary-id"%7D&API_KEY=T0ph%20123!&rawTemporaryId=xyz'; linkUrl=false; selector = '.content'; alt='Щомісячний електронний лист дайджесту зі статистикою коментарів та трьома коментарями, які потребують перегляду, кожен з посиланнями на підтвердження, відповідь та спам'; title='Лист дайджесту' app-screenshot-end]

#### Типи сповіщень

FastComments надсилає кілька типів електронних листів модераторам та адміністраторам. За потреби можна відмовитися від сповіщень `Comment Reply`, залишаючи отримання сповіщень `New Comment`, вибравши відповідні параметри на сторінці `Edit Notifications`, показаній вище.

---