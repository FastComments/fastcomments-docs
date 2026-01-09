[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO использует HMAC-SHA256 шифрование как механизм реализации SSO. Сначала мы рассмотрим общую архитектуру, приведём примеры и детальные шаги.

Также есть документация по миграции от других провайдеров с аналогичными механизмами SSO и по отличиям.

The flow looks like this:

<div class="screenshot white-bg">
    <div class="title">Поток Secure SSO</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Диаграмма Secure SSO" />
</div>

Поскольку Secure SSO требует full-stack разработки, полнофункциональные работающие примеры кода на Java/Spring, NodeJS/Express и чистом PHP в настоящее время <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">на GitHub</a>.

Хотя в примере для NodeJS мы используем ExpressJS, а в примере для Java — Spring, в этих средах для реализации FastComments SSO не требуются дополнительные фреймворки/библиотеки — работают встроенные криптографические пакеты.

Вам не нужно создавать новые API-эндпойнты для FastComments SSO. Просто зашифруйте информацию о пользователе с помощью вашего секретного ключа и передайте полезную нагрузку в виджет комментариев.

#### Получение вашего API Secret Key

Ваш API Secret можно получить на <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">этой странице</a>. Вы также можете найти эту страницу, перейдя в My Account, кликнув по плитке API/SSO и затем нажав "Get API Secret Key".

#### Параметры виджета комментариев

Общая документация по API для виджета комментариев доступна <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">здесь</a>.

Давайте более подробно разберём, что означают эти параметры.

Виджет комментариев принимает объект конфигурации — вы уже передаёте его, если используете FastComments для передачи вашего идентификатора клиента (называемого tenantId).

Чтобы включить SSO, передайте новый объект "sso", который должен содержать следующие параметры. Значения должны генерироваться на стороне сервера.

- userDataJSONBase64: Данные пользователя в формате JSON, затем закодированные в Base64.
- verificationHash: HMAC-SHA256 хэш, созданный из UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Эпохальная временная метка, в **миллисекундах**. Не должна быть в будущем и не более двух дней в прошлом.
- loginURL: URL, который виджет комментариев может показать для входа пользователя.
- logoutURL: URL, который виджет комментариев может показать для выхода пользователя.
- loginCallback: При передаче вместо loginURL — функция, которую виджет комментариев вызовет при клике по кнопке входа.
- logoutCallback: При передаче вместо logoutURL — функция, которую виджет комментариев вызовет при клике по кнопке выхода.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### The User Object

[inline-code-attrs-start title = 'Объект пользователя'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Обязательно. Макс. 1k символов. **/
    id: string;
    /** Обязательно. Макс. 1k символов. Примечание: Должно быть уникальным. **/
    email: string;
    /** Обязательно. Макс. 1k символов. Примечание: Имя пользователя не может быть email. Не обязательно уникально. **/
    username: string;
    /** Необязательно. Макс. 3k символов для URL. По умолчанию используется gravatar на основе email. Поддерживаются изображения, закодированные в base64, в этом случае лимит — 50k символов. **/ 
    avatar?: string;
    /** Необязательно. По умолчанию false. **/
    optedInNotifications?: boolean;
    /** Необязательно. По умолчанию false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Необязательно. Макс. 100 символов. Эта метка будет отображаться рядом с именем. По умолчанию — Администратор/Модератор, если применимо. **/
    displayLabel?: string;
    /** Необязательно. Макс. 500 символов. Это будет показано вместо имени пользователя. **/
    displayName?: string;
    /** Необязательно. Макс. 2k символов. Имя пользователя будет ссылаться на этот URL. **/
    websiteUrl?: string;
    /** Необязательно. До 100 групп на пользователя. ID группы не может быть длиннее 50 символов. **/
    groupIds?: string[];
    /** Необязательно. Обозначает пользователя как администратора. **/
    isAdmin?: boolean;
    /** Необязательно. Обозначает пользователя как модератора. **/
    isModerator?: boolean;
    /** Необязательно, по умолчанию true. Установите в false, чтобы включить вкладку "activity" в профиле пользователя. **/
    isProfileActivityPrivate?: boolean;
    /** Необязательно, по умолчанию false. Установите в true, чтобы отключить комментарии в профиле. **/
    isProfileCommentsPrivate?: boolean;
    /** Необязательно, по умолчанию false. Установите в true, чтобы отключить отправку личных сообщений этому пользователю. **/
    isProfileDMDisabled?: boolean;
}
[inline-code-end]

#### Moderators and Administrators

Для администраторов и модераторов передайте соответствующие флаги `isAdmin` или `isModerator` в объекте `SSOUser`.

#### Notifications

Чтобы включить или отключить уведомления, установите значение `optedInNotifications` в `true` или `false` соответственно. При первом загрузке страницы пользователем с этим значением в SSO-пейлоаде его настройки уведомлений будут обновлены.

Кроме того, если вы хотите, чтобы пользователи получали уведомления по электронной почте о действиях на страницах, на которые они подписаны (в отличие от только внутренних уведомлений в приложении), установите `optedInSubscriptionNotifications` в `true`.

#### VIP Users & Special Labels

Вы можете отображать специальную метку рядом с именем пользователя, используя необязательное поле "displayLabel".

#### Unauthenticated users

Чтобы представить неаутентифицированного пользователя, просто не заполняйте userDataJSONBase64, verificationHash или timestamp. Укажите loginURL.

Эти пользователи не смогут оставлять комментарии; вместо этого им будет показано сообщение о входе (сообщение, ссылка или кнопка, в зависимости от конфигурации).

#### Direct Examples for Serializing and Hashing User Data

Более подробные примеры доступны <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">здесь</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">здесь</a> (java) и <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">здесь</a> (php).

Мы понимаем, что любая интеграция может быть сложным и болезненным процессом. Не стесняйтесь обращаться к вашему представителю или воспользоваться <a href="https://fastcomments.com/auth/my-account/help" target="_blank">страницей поддержки</a>.

---