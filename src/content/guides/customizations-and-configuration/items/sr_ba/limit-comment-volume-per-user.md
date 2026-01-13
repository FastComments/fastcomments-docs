По подразумевању, сваки корисник може послати до `5 comments` у истом минуту.

Ово се прати по user id, anon user id, и ip address (hashed).

Ово се може прилагодити без кода, на страници за прилагођавање видгета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; title='Limiting Comment Volume Per User' app-screenshot-end]

Имајте на уму да, ако користите API за креирање коментара, можда ћете желети да пошаљете оригиналну `ip` адресу корисника у захтеву према нашем backend-у како би се ограничење учесталости примењивало по кориснику, а не глобално за ваш налог.