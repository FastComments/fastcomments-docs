---
Подразумевано, сваки корисник може послати до `5 comments` у истом минуту.

Ово се прати по user id, anon user id, и ip address (hashed).

Ово се може прилагодити без кода, на страници за прилагођавање виџета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; title='Limiting Comment Volume Per User' app-screenshot-end]

Имајте у виду да ако користите comment creation API можда ћете желети да у захтеву пошаљете корисникову оригиналну `ip` адресу нашем backend-у како би rate limiting био примењен по кориснику, а не глобално на ваш налог.

---