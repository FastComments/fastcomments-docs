---
По подразбиране всеки потребител може да изпрати до `5 comments` в същата минута.

Това се проследява чрез user id, anon user id и ip address (hashed).

Това може да се персонализира без код, на страницата за персонализиране на уиджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; alt='Поле за максимален брой коментари за минута на страницата за персонализиране на уиджета, зададено на 5 по подразбиране'; title='Ограничаване на обема на коментарите за потребител' app-screenshot-end]

Обърнете внимание, че ако използвате comment creation API може да искате да предадете оригиналния `ip` address на потребителя в заявката към нашия backend, за да се приложи rate limiting
per user and not globally to your account.
---