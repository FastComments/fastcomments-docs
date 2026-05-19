The FastComments LTI 1.3 інтеграція дотримується принципу найменших привілеїв: вона використовує лише ті дані запуску, які необхідні для ідентифікації користувача, прив'язки коментарів до правильного курсу та ресурсу, а також застосування дозволів на основі ролей.

Решта цієї сторінки відображає кожен клейм, який споживає інтеграція, кожну службу LTI Advantage, яку вона не запитує, і кожну категорію даних, яку вона не збирає. Рецензенти з питань безпеки та закупівель можуть безпосередньо використовувати відповіді з таблиць нижче.

## Дані, що отримуються з LMS

Кожен запуск LTI 1.3 містить підписаний JWT від LMS. FastComments витягує з цього JWT такі клейми і нічого більше:

| Field | LTI claim | Purpose | Required | Stored |
|-------|-----------|---------|----------|--------|
| User identifier | `sub` | Визначає користувача послідовно між запусками, щоб одна й та сама особа відповідала одному SSO-користувачу FastComments | Yes | Yes, as part of a stable internal SSO ID |
| Display name | `name` | Підпис, що відображається поруч із коментарями користувача | Yes (falls back to "LMS User" if absent) | Yes |
| Email | `email` | Збіг акаунту, повідомлення, модерація, кореспонденція з підтримкою | Optional (the integration works without it) | Yes when provided |
| Avatar URL | `picture` | Відображається поруч із коментарями користувача | Optional | URL only; FastComments does not download or rehost the image |
| Roles | `https://purl.imsglobal.org/spec/lti/claim/roles` | Визначає, чи є користувач адміністратором, викладачем (модератором) чи учнем | Yes | Derived `isAdmin` / `isModerator` flags on the SSO session |
| Course context | `https://purl.imsglobal.org/spec/lti/claim/context` (`id`, `title`) | Зв'язує потік коментарів з правильним курсом у LMS | Yes | Yes, as part of the resolved page identifier |
| Resource link | `https://purl.imsglobal.org/spec/lti/claim/resource_link` (`id`) | Прив'язує коментарі до правильної активності або розміщення інструменту в курсі | Yes when present | Yes, as part of the resolved page identifier |
| Deployment ID | `https://purl.imsglobal.org/spec/lti/claim/deployment_id` | Направляє запуск до правильної конфігурації орендаря FastComments | Yes | Yes, on the FastComments LTI configuration record |

## Клейми та області, заявлені при реєстрації

Під час динамічної реєстрації LTI 1.3 FastComments реєструється з `scope: ""` (без додаткових OAuth-областей) і заявляє лише ці OpenID Connect клейми:

`iss`, `sub`, `name`, `email`, `picture`

Вона реєструє два типи повідомлень:

- `LtiResourceLinkRequest` - стандартний запуск курсу у FastComments.
- `LtiDeepLinkingRequest` - дозволяє викладачам розміщувати інструмент FastComments у курсі.

Жодних додаткових токенів доступу від LMS не запитується.

## Служби LTI Advantage, які не запитуються

| Service / scope | Requested? | Reason |
|------------------|------------|--------|
| Names and Role Provisioning Services (NRPS) | No | Інтеграції не потрібен список учасників курсу; ідентичність користувача надходить з кожним запуском |
| Assignment and Grade Services (AGS) - lineitem, score, result scopes | No | Інтеграція не працює з журналом оцінок |
| Deep Linking beyond the standard placement return | No additional data | Deep linking використовується лише для розміщення інструменту викладачем; вміст курсу не перелічується |

## Дані, які не збираються

Окрім самих даних LTI, FastComments не запитує і не отримує від LMS або користувача наступне:

| Category | Collected? |
|----------|------------|
| Student grades | No |
| Assignment submissions | No |
| Attendance records | No |
| Full course rosters | No |
| Government identifiers | No |
| Date of birth | No |
| Postal address or phone number | No |
| Financial information | No |
| LMS administrator credentials | No |

## Межі доступу

- FastComments отримує дані лише в межах авторизованого запуску LTI 1.3, підписаного ключами LMS, зареєстрованими в системі. Інтеграція не робить зворотних викликів у LMS для отримання додаткової інформації.
- Токени запуску одноразові й мають короткий термін дії. Повторні або прострочені токени відхиляються.
- Адміністратори LMS контролюють, де інструмент розгортається в їхній платформі. Наприклад, D2L Brightspace підтримує обмеження по org-unit для кожного розгортання та налаштування безпеки на рівні розгортання, що дозволяє адміністраторам обмежити доступ до інструменту конкретними курсами або організаційними одиницями замість глобальної доступності. Moodle, Blackboard, Sakai і Schoology пропонують еквівалентні контролі на рівні розгортання у своїх реалізаціях LTI 1.3.

## Зберігання та збереження

FastComments зберігає дані, похідні від LTI, протягом періоду активного коментування та відповідно до налаштувань збереження, визначених клієнтом. Дані коментарів зберігаються у виробничому сховищі з шифруванням at-rest. Після закриття акаунта або надісланого письмового запиту на видалення FastComments видаляє або анонімізує дані клієнта відповідно до застосовної угоди.

Для повних відомостей про зберігання та обробку даних див. <a href="https://fastcomments.com/privacy-policy" target="_blank">Політика конфіденційності FastComments</a>.

## Періодичність перегляду

Будь-яка нова функція LTI, яка вимагатиме додаткових клеймів, областей або служб LTI Advantage, переглядається перед випуском, щоб підтвердити, що запитуваний доступ необхідний і пропорційний функції, яка відправляється.

## Коротка відповідь для анкет з безпеки

> FastComments застосовує принцип найменших привілеїв і мінімізації даних до своєї інтеграції LTI 1.3. Інтеграція використовує лише клейми запуску LTI, необхідні для автентифікації користувача (`sub`, `name`, `email`, `picture`), визначення його ролі та ідентифікації курсу й ресурсу, до яких належать коментарі. FastComments не запитує Names and Role Provisioning Services, Assignment and Grade Services, дані журналу оцінок, відвідуваність, повні списки учасників або адміністративний доступ до LMS. Адміністратори LMS зберігають контроль над тим, в яких організаційних одиницях, курсах і розгортаннях інструмент доступний.