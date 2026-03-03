[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO використовує шифрування HMAC-SHA256 як механізм реалізації SSO. Спочатку ми розглянемо загальну архітектуру, наведемо приклади та детальні кроки.

Також є документація щодо міграції від інших провайдерів із подібними механізмами SSO та про відмінності.

The flow looks like this:

<div class="screenshot white-bg">
    <div class="title">Потік Secure SSO</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Діаграма Secure SSO" />
</div>

Оскільки Secure SSO включає full-stack розробку, повні робочі приклади коду на Java/Spring, NodeJS/Express та vanilla PHP наразі розміщені <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">на GitHub</a>.

Хоча в прикладі для NodeJS ми використовуємо ExpressJS, а в прикладі для Java — Spring, у цих середовищах для реалізації FastComments SSO не потрібно жодних фреймворків/бібліотек — працюють вбудовані криптографічні пакети.

Вам не потрібно створювати нові API-ендпоїнти для FastComments SSO. Просто зашифруйте інформацію про користувача за допомогою вашого секретного ключа та передайте payload у віджет коментарів.

#### Отримайте свій секретний ключ API

Ваш API Secret можна отримати з цієї <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">сторінки</a>. Ви також можете знайти цю сторінку, перейшовши до My Account, клацнувши плитку API/SSO, а потім натиснувши «Отримати секретний ключ API».

#### Параметри віджета коментарів

High-level API documentation for the comment widget can be found <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">here</a>.

Давайте детальніше розглянемо, що означають ці параметри.

Віджет коментарів приймає об'єкт конфігурації - ви вже передаєте цей об'єкт, якщо використовуєте FastComments для передачі вашого customer id (названого tenantId).

Щоб увімкнути SSO, передайте новий "sso" об'єкт, який повинен мати наступні параметри. Значення повинні генеруватися на стороні сервера.

- userDataJSONBase64: Дані користувача у форматі JSON, які потім кодуються Base64.
- verificationHash: HMAC-SHA256 хеш, створений з UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Epoch timestamp, in **milliseconds**. Не має бути датованою в майбутньому або більш ніж на два дні в минулому.
- loginURL: URL, який віджет коментарів може показати для входу користувача.
- logoutURL: URL, який віджет коментарів може показати для виходу користувача.
- loginCallback: Коли надано замість login URL, функція, яку віджет коментарів викличе при натисканні кнопки входу.
- logoutCallback: Коли надано замість logout URL, функція, яку віджет коментарів викличе при натисканні кнопки виходу.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### Об'єкт користувача
[inline-code-attrs-start title = 'Об\'єкт користувача'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Обов'язково. Максимум 1k символів. **/
    id: string;
    /** Обов'язково. Максимум 1k символів. Примітка: має бути унікальним. **/
    email: string;
    /** Обов'язково. Максимум 1k символів. Примітка: ім'я користувача не може бути email. Не обов'язково бути унікальним. **/
    username: string;
    /** Необов'язково. Максимум 3k символів для URL. За замовчуванням береться з gravatar на основі email. Підтримує 64 закодовані зображення; у такому разі ліміт — 50k символів. **/ 
    avatar?: string;
    /** Необов'язково. За замовчуванням false. **/
    optedInNotifications?: boolean;
    /** Необов'язково. За замовчуванням false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Необов'язково. Максимум 100 символів. Ця мітка відображатиметься поруч з їхнім ім'ям. За замовчуванням — Administrator/Moderator коли застосовно. **/
    displayLabel?: string;
    /** Необов'язково. Максимум 500 символів. Це відображатиметься замість username. **/
    displayName?: string;
    /** Необов'язково. Максимум 2k символів. Ім'я користувача буде посиланням на це. **/
    websiteUrl?: string;
    /** Необов'язково. До 100 груп на користувача. Ідентифікатор групи не може бути довшим за 50 символів. **/
    groupIds?: string[];
    /** Необов'язково. Позначає користувача як адміністратора. **/
    isAdmin?: boolean;
    /** Необов'язково. Позначає користувача як модератора. **/
    isModerator?: boolean;
    /** Необов'язково, за замовчуванням true. Встановіть у false, щоб увімкнути вкладку "activity" у профілі користувача. **/
    isProfileActivityPrivate?: boolean;
    /** Необов'язково, за замовчуванням false. Встановіть у true, щоб вимкнути коментарі в профілі. **/
    isProfileCommentsPrivate?: boolean;
    /** Необов'язково, за замовчуванням false. Встановіть у true, щоб вимкнути прямі повідомлення цьому користувачу. **/
    isProfileDMDisabled?: boolean;
    /** Необов'язкова конфігурація для бейджів користувача. **/
    badgeConfig?: {
        /** Масив глобальних ID значків для призначення. Обмеження — 30 значків. Порядок зберігається. **/
        badgeIds: string[];
        /** Масив ID значків, прив'язаних до поточної сторінки (urlId). Відображаються тільки на призначеній сторінці. **/
        pageBadgeIds?: string[];
        /** Якщо true, замінює існуючі відображувані значки. Глобальні та сторінково-прив'язані значки перекриваються незалежно. **/
        override?: boolean;
        /** Якщо true, оновлює властивості відображення значків відповідно до конфігурації tenant. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Модератори та Адміністратори

Для адмінів та модераторів передайте відповідні прапорці `isAdmin` або `isModerator` в об'єкті `SSOUser`.

#### Сповіщення

Щоб увімкнути або вимкнути сповіщення, встановіть значення `optedInNotifications` відповідно в `true` або `false`. Першого разу, коли користувач завантажить сторінку з цим значенням у SSO-пейлоуді, його налаштування сповіщень будуть оновлені.

Крім того, якщо ви хочете, щоб користувачі отримували електронні листи зі сповіщеннями про активність на сторінках, на які вони підписані (на відміну від лише внутрішніх сповіщень), встановіть `optedInSubscriptionNotifications` у `true`.

#### VIP-користувачі та спеціальні мітки

Ви можете відобразити спеціальну мітку поруч із ім'ям користувача, використавши необов'язкове поле "displayLabel".

#### Неавторизовані користувачі

Щоб представити неавторизованого користувача, просто не заповнюйте userDataJSONBase64, verificationHash або timestamp. Надайте loginURL.

Ці користувачі не зможуть залишати коментарі; натомість їм буде показано повідомлення про вхід (повідомлення, посилання або кнопка, залежно від конфігурації).

#### Прямі приклади серіалізації та хешування даних користувача

Більше деталей та прикладів можна знайти <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">тут</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">тут</a> (java) та <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">тут</a> (php).

Ми розуміємо, що будь-яка інтеграція може бути складним і болісним процесом. Не соромтеся звертатися до вашого представника або скористатися <a href="https://fastcomments.com/auth/my-account/help" target="_blank">сторінкою підтримки</a>.

---