Существует два способа запретить пользователям комментировать на вашем сайте с помощью FastComments.

Первый: если вы уже знаете их электронную почту, вы можете ввести её на странице <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">заблокированных пользователей</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

К этой странице можно получить доступ через Moderate Comments -> Banned Users

Когда мы блокируем пользователя, можно выбрать тип, либо Permanent, либо Permanent Shadow Ban:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

Второй способ заблокировать пользователя — нажать кнопку бана, которая размещена у каждого комментария на странице модерации комментариев.

При нажатии кнопки бана появятся параметры, где можно указать тип и длительность бана.

### Алиасы электронной почты

При блокировке пользователя по электронной почте FastComments автоматически игнорирует `+` алиасы. Например, блокировка `user+alias@gmail.com` также заблокирует `user@gmail.com` и любую другую `+` вариацию этого адреса, такую как `user+other@gmail.com`.

### Теневые баны

Теневой бан — это тип блокировки, который создаёт видимость того, что комментарий или голос пользователя были успешно сохранены, хотя на самом деле это не так. Это может быть желаемым поведением в определённых ситуациях.

### Блокировка по IP-адресу

Если тенант не пожелает отказаться, FastComments поддерживает блокировку по IP, сохраняя хешированную версию IP-адреса комментатора.