[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO використовує шифрування HMAC-SHA256 як механізм реалізації SSO. Спочатку ми розглянемо загальну архітектуру, наведемо приклади та детальні кроки.

Також є документація щодо міграції від інших провайдерів з подібними механізмами SSO та відмінностей.

Потік виглядає так:

<div class="screenshot white-bg">
    <div class="title">Потік Secure SSO</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Схема Secure SSO" />
</div>

Оскільки Secure SSO передбачає розробку повного стеку, повні робочі приклади коду на Java/Spring, NodeJS/Express і чистому PHP наразі розміщені <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">на GitHub</a>.

Хоча в прикладі для NodeJS ми використовуємо ExpressJS, а в Java — Spring, у цих середовищах для реалізації FastComments SSO не потрібні додаткові фреймворки/бібліотеки — працюють вбудовані криптографічні пакети.

Вам не потрібно створювати нові API-ендпоінти для FastComments SSO. Просто зашифруйте інформацію про користувача за допомогою вашого секретного ключа і передайте payload у віджет коментарів.

#### Отримайте ваш секретний ключ API

Ваш секретний ключ API можна отримати з <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">цієї сторінки</a>. Ви також можете знайти цю сторінку, перейшовши в My Account, натиснувши плитку API/SSO, а потім натиснувши «Get API Secret Key».

#### Параметри віджета коментарів

Документація високого рівня по API для віджета коментарів доступна <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">тут</a>.

Давайте розглянемо детальніше, що означають ці параметри.

Віджет коментарів приймає конфігураційний об'єкт — ви вже передаєте його, якщо використовуєте FastComments для передачі вашого ідентифікатора клієнта (названого tenantId).

Щоб увімкнути SSO, передайте новий об'єкт "sso", який має містити такі параметри. Значення повинні генеруватися на сервері.

- userDataJSONBase64: Дані користувача у форматі JSON, які потім кодуються у Base64.
- verificationHash: HMAC-SHA256 хеш, створений з UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Епохальний часовий штамп у **мілісекундах**. Не повинен бути у майбутньому або більше ніж на два дні в минулому.
- loginURL: URL, який віджет коментарів може показати для входу користувача.
- logoutURL: URL, який віджет коментарів може показати для виходу користувача.
- loginCallback: Коли вказано замість loginURL, функція, яку віджет коментарів викличе при натисканні кнопки входу.
- logoutCallback: Коли вказано замість logoutURL, функція, яку віджет коментарів викличе при натисканні кнопки виходу.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### The User Object

[inline-code-attrs-start title = 'Об’єкт користувача'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Обов'язково. Максимум 1к символів. **/
    id: string;
    /** Обов'язково. Максимум 1к символів. Примітка: Має бути унікальним. **/
    email: string;
    /** Обов'язково. Максимум 1к символів. Примітка: Ім'я користувача не може бути електронною поштою. Не має бути унікальним. **/
    username: string;
    /** Необов'язково. Максимум 3к символів для URL. За замовчуванням береться з gravatar на основі email. Підтримує 64-кодовані зображення, у такому випадку ліміт — 50к символів. **/ 
    avatar?: string;
    /** Необов'язково. За замовчуванням — false. **/
    optedInNotifications?: boolean;
    /** Необов'язково. За замовчуванням — false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Необов'язково. Максимум 100 символів. Ця мітка буде показана поруч із їхнім ім'ям. За замовчуванням — Адміністратор/Модератор, коли застосовується. **/
    displayLabel?: string;
    /** Необов'язково. Максимум 500 символів. Це буде показано замість імені користувача. **/
    displayName?: string;
    /** Необов'язково. Максимум 2к символів. Ім'я користувача буде посилатися сюди. **/
    websiteUrl?: string;
    /** Необов'язково. До 100 груп на користувача. Ідентифікатор групи не може бути довшим за 50 символів. **/
    groupIds?: string[];
    /** Необов'язково. Позначає користувача як адміністратора. **/
    isAdmin?: boolean;
    /** Необов'язково. Позначає користувача як модератора. **/
    isModerator?: boolean;
    /** Необов'язково, за замовчуванням true. Встановіть у false, щоб увімкнути вкладку "активність" у профілі користувача. **/
    isProfileActivityPrivate?: boolean;
    /** Необов'язково, за замовчуванням false. Встановіть у true, щоб вимкнути коментарі в профілі. **/
    isProfileCommentsPrivate?: boolean;
    /** Необов'язково, за замовчуванням false. Встановіть у true, щоб вимкнути приватні повідомлення цьому користувачу. **/
    isProfileDMDisabled?: boolean;
}
[inline-code-end]

#### Moderators and Administrators

Для адміністраторів і модераторів передайте відповідні прапорці `isAdmin` або `isModerator` в об'єкті `SSOUser`.

#### Notifications

Щоб увімкнути або вимкнути сповіщення, встановіть значення `optedInNotifications` в `true` або `false` відповідно. Перший раз, коли користувач завантажить сторінку з цим значенням у SSO-пейлоаді, його налаштування сповіщень будуть оновлені.

Крім того, якщо ви хочете, щоб користувачі отримували електронні листи про активність на сторінках, на які вони підписані (на відміну від тільки внутрішніх сповіщень у додатку), встановіть `optedInSubscriptionNotifications` в `true`.

#### VIP Users & Special Labels

Ви можете відобразити спеціальну мітку поруч із іменем користувача, використавши необов'язкове поле "displayLabel".

#### Неавторизовані користувачі

Щоб представити неавторизованого користувача, просто не заповнюйте userDataJSONBase64, verificationHash або timestamp. Надайте loginURL.

Ці користувачі не зможуть залишати коментарі, натомість їм буде показано повідомлення про вхід (повідомлення, посилання або кнопка, залежно від конфігурації).

#### Прямі приклади серіалізації та хешування даних користувача

Додаткові деталі та приклади доступні <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">тут</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">тут</a> (java) та <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">тут</a> (php).

Ми розуміємо, що будь-яка інтеграція може бути складним і болісним процесом. Не вагайтеся звертатися до вашого представника або використовувати <a href="https://fastcomments.com/auth/my-account/help" target="_blank">сторінку підтримки</a>.