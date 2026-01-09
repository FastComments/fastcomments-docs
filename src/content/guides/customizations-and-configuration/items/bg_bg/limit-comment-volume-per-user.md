По подразбиране всеки потребител може да изпрати до `5 comments` в същата минута.

Това се проследява по user id, anon user id, and ip address (hashed).

Това може да се персонализира без код, на страницата за персонализиране на widget-а:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; title='Limiting Comment Volume Per User' app-screenshot-end]

Имайте предвид, че ако използвате API за създаване на коментари, може да искате да предадете оригиналния `ip` адрес на потребителя в заявката към нашия бекенд, така че ограничаването по честота да се прилага на потребител, а не глобално за вашия акаунт.