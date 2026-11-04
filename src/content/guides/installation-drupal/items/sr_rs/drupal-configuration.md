Сва подешавања се налазе под `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Обавезно

- **Tenant ID** - Ваш FastComments Tenant ID. Пронађите га под [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Обавезан за Secure SSO, верификацију webhook-ова и синхронизацију страница. Налази се под [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Чување API Secret-а

Модул чита API Secret из првог од ових извора који има вредност:

1. **Key модул** - Инсталирајте [Key](https://www.drupal.org/project/key), креирајте *Authentication* кључ и изаберите га под **API Secret key** на форми за подешавања. Кључ може бити подржан променљивом окружења или фајлом изван web root-а.
2. **settings.php** - Додајте `$config['fastcomments.settings']['api_secret'] = 'your-secret';` у `settings.php` или `settings.local.php`. Тајна никада не улази у извезену конфигурацију.
3. **Форма за подешавања** - Унесите је у поље **API Secret**. Ово чува вредност у `fastcomments.settings`, тако да завршава у извозима конфигурације. Drupal извештај о статусу упозорава када је ово активни извор.

Форма приказује који је извор активан. Остављање поља празним задржава сачувану тајну. Штиклирајте **Remove the API Secret stored in configuration** да је обришете након премештања тајне у Key модул или `settings.php`.

## Стил коментарисања

Изаберите виџет који одговара начину на који желите да људи комуницирају на вашем сајту.

- **Live Comments** - Коментари у реалном времену са гранањем.
- **Streaming Chat** - Интерфејс за живи ћаскање, добар за догађаје и преносе уживо.
- **Collab Chat** - Анотација изабраног текста на главном подручју садржаја. Посетиоци истичу текст и покрећу дискусију у контексту.
- **Collab Chat + Comments** - И collab ћаскање и стандардни коментари на истој страници.

## SSO режим

- **None** - Без SSO-а. Корисници коментаришу као гости или креирају FastComments налог.
- **Simple** - Прослеђује Drupal информације о кориснику (име, имејл, аватар) FastComments-у без серверске верификације.
- **Secure** - Користи HMAC-SHA256 за верификацију Drupal корисника са FastComments-ом. Препоручено када имате конфигурисан API Secret.

Погледајте одељак `Single Sign-On (SSO)` за детаље.

## Остала подешавања

- **CDN URL** - Подразумевано `https://cdn.fastcomments.com`.
- **Site URL** - Подразумевано `https://fastcomments.com`.
- **Email notifications** - Шаље имејл аутору садржаја када је нови коментар објављен на његовом садржају.

За EU data residency, погледајте одељак `EU Data Residency`.