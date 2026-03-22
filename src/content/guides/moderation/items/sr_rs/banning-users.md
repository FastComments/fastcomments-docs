Постоје два начина да забраните корисницима да коментаришу на вашем сајту помоћу FastComments.

Први начин је: ако већ знате њихову е-пошту, можете је унети на страницу <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">забрањених корисника</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

Ова страница је доступна преко Модерирање коментара -> Забрањени корисници

Када желимо да забранимо корисника, можемо изабрати тип: или Permanent или Permanent Shadow Ban:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

Други начин да забраните корисника је кликом на дугме за забрану које се налази на сваком коментару на страници модерирања коментара.

Када кликнете на дугме за забрану, биће вам приказане опције у којима можете одредити тип забране и трајање.

### Алијаси е-поште

Када забрањујете корисника по е-пошти, FastComments аутоматски игнорише `+` алијасе. На пример, забрана `user+alias@gmail.com` ће такође забранити `user@gmail.com` и било коју другу `+` варијацију те адресе, као што је `user+other@gmail.com`.

### Скривене забране

Скривена забрана је тип забране који даје утисак да је кориснички коментар или глас успешно сачуван, иако то у ствари није било. Ово може бити пожељно у одређеним ситуацијама.

### Забрана преко IP адресе

Осим ако корисник налога не жели да се искључи, FastComments подржава забрану по IP-у тако што чува хеширану верзију IP адресе коментатора.