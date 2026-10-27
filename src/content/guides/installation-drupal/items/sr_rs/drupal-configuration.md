Сва подешавања се налазе под `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Required

- **Tenant ID** - Ваш FastComments Tenant ID. Нађите га у [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Потребан за Secure SSO, верификацију веб‑хукова и синхронизацију страница. Налази се у [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Storing the API Secret

Модул чита API Secret из првог од ових извора који има вредност:

1. **Key module** - Инсталирајте [Key](https://www.drupal.org/project/key), направите кључ *Authentication*, и изаберите га под **API Secret key** у формулару подешавања. Кључ може бити подржан променљивом окружења или датотеком изван веб‑корена.
2. **settings.php** - Додајте `$config['fastcomments.settings']['api_secret'] = 'your-secret';` у `settings.php` или `settings.local.php`. Тајна никада не улази у извезену конфигурацију.
3. **The settings form** - Унесите је у поље **API Secret**. Ово чува вредност у `fastcomments.settings`, тако да се појављује у извозима конфигурације. Drupal извештај о статусу упозорава када је ово активни извор.

Формулар приказује који извор је активан. Остављање поља празним задржава сачувану тајну. Означите **Remove the API Secret stored in configuration** да је избришете након што преместите тајну у Key модул или `settings.php`.

## Commenting Style

Изаберите виџет који одговара начину на који желите да људи комуницирају на вашем сајту.

- **Live Comments** - Коментари у реалном времену са нити.
- **Streaming Chat** - Интерфејс за живи ћаскање, погодан за догађаје и уживо преносе.
- **Collab Chat** - Анотација одабраног текста у главном простору садржаја. Посетиоци истичу текст и покрећу дискусију у контексту.
- **Collab Chat + Comments** - И Collab Chat и стандардни коментари на истој страници.

## SSO Mode

- **None** - Без SSO. Корисници коментаришу као гости или праве FastComments налог.
- **Simple** - Прослеђује информације о Drupal кориснику (име, имејл, аватар) у FastComments без верификације на серверу.
- **Secure** - Користи HMAC‑SHA256 за верификацију Drupal корисника у FastComments. Препоручено када имате подешен API Secret.

Погледајте одељак `Single Sign-On (SSO)` за детаље.

## Other Settings

- **CDN URL** - Подразумевано је `https://cdn.fastcomments.com`.
- **Site URL** - Подразумевано је `https://fastcomments.com`.
- **Email notifications** - Пошаљите имејл аутору садржаја када се на њиховом садржају објави нови коментар.

За резиденцију података у ЕУ, погледајте одељак `EU Data Residency`.