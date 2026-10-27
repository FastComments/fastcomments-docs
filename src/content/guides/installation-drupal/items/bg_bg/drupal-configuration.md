All settings live under `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Задължително

- **Tenant ID** - Вашият FastComments Tenant ID. Намерете го в [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Необходим за Secure SSO, проверка на webhook и синхронизация на страници. Намира се в [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Съхраняване на API Secret

Модулът чете API Secret от първия от тези източници, който има стойност:

1. **Key module** - Инсталирайте [Key](https://www.drupal.org/project/key), създайте ключ за *Authentication* и го изберете под **API Secret key** във формуляра за настройки. Ключът може да бъде подкрепен с променлива на средата или файл извън уеб корена.
2. **settings.php** - Добавете `$config['fastcomments.settings']['api_secret'] = 'your-secret';` към `settings.php` или `settings.local.php`. Тайната никога не попада в експортираната конфигурация.
3. **The settings form** - Въведете я в полето **API Secret**. Това съхранява стойността в `fastcomments.settings`, така че тя се включва в експортираната конфигурация. Съобщението за състоянието в Drupal предупреждава, когато това е активният източник.

Формулярът показва кой източник е активен. Оставянето на полето празно запазва съхранената тайна. Поставете отметка на **Премахнете API Secret, съхранен в конфигурацията**, за да я изтриете след преместване на тайната към Key модула или `settings.php`.

## Стил на коментиране

Изберете уиджета, който отговаря на начина, по който искате хората да разговарят на вашия сайт.

- **Live Comments** - Коментари в реално време с нишки.
- **Streaming Chat** - Интерфейс за жив чат, подходящ за събития и живи предавания.
- **Collab Chat** - Анотация чрез избор на текст в основната област на съдържанието. Посетителите маркират текст и започват дискусия в контекст.
- **Collab Chat + Comments** - И колаб чат, и стандартни коментари на същата страница.

## Режим SSO

- **None** - Без SSO. Потребителите коментират като гости или създават FastComments акаунт.
- **Simple** - Предава информация за потребителите от Drupal (име, имейл, аватар) към FastComments без проверка от сървъра.
- **Secure** - Използва HMAC-SHA256 за проверка на потребителите от Drupal с FastComments. Препоръчва се, когато имате конфигуриран API Secret.

Вижте раздела `Single Sign-On (SSO)` за подробности.

## Други настройки

- **CDN URL** - По подразбиране `https://cdn.fastcomments.com`.
- **Site URL** - По подразбиране `https://fastcomments.com`.
- **Email notifications** - Изпраща имейл до автора на съдържанието, когато нов коментар бъде публикуван върху неговото съдържание.

За резидентност на данни в ЕС, вижте раздела `EU Data Residency`.