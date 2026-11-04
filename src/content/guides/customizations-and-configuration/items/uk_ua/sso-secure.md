[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO використовує шифрування HMAC-SHA256 як механізм реалізації SSO. Спочатку ми розглянемо загальну архітектуру, надамо приклади та детальні кроки.

Також є документація щодо міграції від інших провайдерів зі схожими механізмами SSO та їхніх відмінностей.

Потік виглядає так:

<div class="screenshot white-bg">
    <div class="title">Потік Secure SSO</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Діаграма Secure SSO" />
</div>

Оскільки Secure SSO передбачає full-stack розробку, повні робочі приклади коду на Java/Spring, NodeJS/Express та чистому PHP наразі доступні <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">на GitHub</a>.

Хоча ми використовуємо ExpressJS у прикладі для NodeJS та Spring у прикладі для Java, для реалізації FastComments SSO в цих середовищах не потрібні жодні фреймворки/бібліотеки — працюють нативні криптографічні пакети.

Вам не потрібно писати жодних нових API-ендпоінтів з FastComments SSO. Просто зашифруйте інформацію користувача за допомогою вашого секретного ключа та передайте payload віджету коментарів.

#### Отримайте ваш API Secret Key

Ваш API Secret можна отримати на <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">цій сторінці</a>. Цю сторінку також можна знайти, перейшовши до My Account, натиснувши плитку API/SSO, а потім «Get API Secret Key».

#### Параметри віджета коментарів

Документація API високого рівня для віджета коментарів доступна <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">тут</a>.

Розглянемо детальніше, що означають ці параметри.

Віджет коментарів приймає об'єкт конфігурації — ви вже передаєте його, якщо використовуєте FastComments для передачі вашого customer id (який називається tenantId).

Щоб увімкнути SSO, передайте новий об'єкт «sso», який повинен мати наступні параметри. Значення повинні генеруватися на сервері.

- userDataJSONBase64: Дані користувача у форматі JSON, які потім кодуються в Base64.
- verificationHash: Хеш HMAC-SHA256, створений з UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Мітка часу Epoch, у **мілісекундах**. Не повинна бути в майбутньому або більш ніж на два дні в минулому.
- loginURL: URL, який віджет коментарів може показати для входу користувача.
- logoutURL: URL, який віджет коментарів може показати для виходу користувача.
- loginCallback: Якщо надано замість login URL, функція, яку віджет коментарів викличе при натисканні кнопки входу.
- logoutCallback: Якщо надано замість logout URL, функція, яку віджет коментарів викличе при натисканні кнопки виходу.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Клієнтський код Secure SSO'; isFunctional = false; code-example-end]

#### Об'єкт User

Об'єкт User містить наступну схему:
[inline-code-attrs-start title = 'Об\'єкт User'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Обов'язково. Максимум 1k символів. **/
    id: string;
    /** Обов'язково. Максимум 1k символів. Примітка: Повинен бути унікальним. **/
    email: string;
    /** Обов'язково. Максимум 1k символів. Примітка: Ім'я користувача не може бути email. Не обов'язково унікальне. **/
    username: string;
    /** Необов'язково. Максимум 3k символів для URL. За замовчуванням береться з gravatar на основі email. Підтримує 64 закодовані зображення, у такому випадку ліміт — 50k символів. **/ 
    avatar?: string;
    /** Необов'язково. За замовчуванням false. **/
    optedInNotifications?: boolean;
    /** Необов'язково. За замовчуванням false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Необов'язково. Максимум 100 символів. Ця мітка буде показана поруч з ім'ям. За замовчуванням Administrator/Moderator, якщо застосовно. **/
    displayLabel?: string;
    /** Необов'язково. Максимум 500 символів. Це буде показано замість імені користувача. **/
    displayName?: string;
    /** Необов'язково. Максимум 2k символів. Ім'я користувача буде посиланням на це. **/
    websiteUrl?: string;
    /** Необов'язково. До 100 груп на користувача. Ідентифікатор групи не може бути довшим за 50 символів. **/
    groupIds?: string[];
    /** Необов'язково. Позначає користувача як адміністратора. **/
    isAdmin?: boolean;
    /** Необов'язково. Позначає користувача як модератора. **/
    isModerator?: boolean;
    /** Необов'язково, за замовчуванням true. Встановіть false, щоб увімкнути вкладку «activity» у профілі користувача. **/
    isProfileActivityPrivate?: boolean;
    /** Необов'язково, за замовчуванням false. Встановіть true, щоб вимкнути коментарі профілю. **/
    isProfileCommentsPrivate?: boolean;
    /** Необов'язково, за замовчуванням false. Встановіть true, щоб вимкнути прямі повідомлення цьому користувачу. **/
    isProfileDMDisabled?: boolean;
    /** Необов'язкова конфігурація для значків користувача. **/
    badgeConfig?: {
        /** Масив глобальних ідентифікаторів значків для призначення. Обмежено 30 значками. Порядок зберігається. **/
        badgeIds: string[];
        /** Масив ідентифікаторів значків, обмежених поточною сторінкою (urlId). Відображаються лише на призначеній сторінці. **/
        pageBadgeIds?: string[];
        /** Якщо true, замінює наявні відображені значки. Глобальні та обмежені сторінкою перевизначаються незалежно. **/
        override?: boolean;
        /** Якщо true, оновлює властивості відображення значків з конфігурації tenant. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Модератори та адміністратори

Для адміністраторів та модераторів передайте відповідні прапорці `isAdmin` або `isModerator` в об'єкті `SSOUser`.

#### Сповіщення

Щоб увімкнути або вимкнути сповіщення, встановіть значення `optedInNotifications` відповідно на `true` або `false`. Перший раз, коли користувач завантажить сторінку з цим значенням у SSO payload, його налаштування сповіщень будуть оновлені.

Крім того, якщо ви хочете, щоб користувачі отримували email-сповіщення про активність на сторінках, на які вони підписалися (на відміну від лише сповіщень у застосунку), встановіть `optedInSubscriptionNotifications` на `true`.

#### VIP-користувачі та спеціальні мітки

Ви можете відобразити спеціальну мітку поруч з ім'ям користувача, використовуючи необов'язкове поле «displayLabel».

#### Неавтентифіковані користувачі

Щоб представити неавтентифікованого користувача, просто не заповнюйте userDataJSONBase64, verificationHash або timestamp. Надайте loginURL.

Такі користувачі не зможуть коментувати, і замість цього їм буде показано повідомлення про вхід (повідомлення, посилання або кнопка, залежно від конфігурації).

#### Прямі приклади серіалізації та хешування даних користувача

Більше деталей у вигляді прикладів <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/node-express/routes/index.js#L23" target="_blank">тут</a> (js), <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/java-springboot/src/main/java/com/winricklabs/ssodemo/DemoController.java#L68" target="_blank">тут</a> (java) та <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">тут</a> (php).

Ми розуміємо, що будь-яка інтеграція може бути складним і болючим процесом. Не соромтеся звертатися до вашого представника або використовувати <a href="https://fastcomments.com/auth/my-account/help" target="_blank">сторінку підтримки</a>.