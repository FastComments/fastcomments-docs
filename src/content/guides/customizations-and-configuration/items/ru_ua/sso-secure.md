[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO использует HMAC-SHA256 в качестве механизма для реализации SSO. Сначала мы рассмотрим общую архитектуру, приведём примеры и детальные шаги.

Также есть документация по миграции с других провайдеров с похожими SSO-механизмами и описаны различия.

Поток выглядит следующим образом:

<div class="screenshot white-bg">
    <div class="title">Secure SSO Flow</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Secure SSO Diagram" />
</div>

Поскольку Secure SSO включает full-stack разработку, рабочие примеры кода на Java/Spring, NodeJS/Express и чистом PHP в данный момент находятся <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">на GitHub</a>.

Хотя в примере NodeJS мы используем ExpressJS, а в примере Java — Spring, в этих рантаймах не требуются какие-либо фреймворки/библиотеки для реализации FastComments SSO — работают встроенные криптопакеты.

Вам не нужно писать какие-либо новые API-эндпоинты для FastComments SSO. Просто зашифруйте информацию о пользователе, используя ваш секретный ключ, и передайте payload в виджет комментариев.

#### Получение вашего секретного ключа API

Ваш API Secret можно получить на <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">этой странице</a>. Эту страницу также можно найти, перейдя в My Account, кликнув плитку API/SSO и затем нажав "Get API Secret Key".

#### Параметры виджета комментариев

Высокоуровневая документация API для виджета комментариев доступна <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">здесь</a>.

Давайте подробно разберём, что означают эти параметры.

Виджет комментариев принимает объект конфигурации — вы уже передаёте его, если используете FastComments для передачи вашего tenantId.

Чтобы включить SSO, передайте новый объект "sso", который должен содержать следующие параметры. Значения должны генерироваться на стороне сервера.

- userDataJSONBase64: Данные пользователя в формате JSON, затем закодированные в Base64.
- verificationHash: HMAC-SHA256 хеш, созданный из UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Эпохальная метка времени в **миллисекундах**. Не должна быть в будущем и не более двух дней в прошлом.
- loginURL: URL, который виджет комментариев может показать для входа пользователя.
- logoutURL: URL, который виджет комментариев может показать для выхода пользователя.
- loginCallback: При передаче вместо login URL — функция, которую виджет вызовет при клике на кнопку входа.
- logoutCallback: При передаче вместо logout URL — функция, которую виджет вызовет при клике на кнопку выхода.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### Объект пользователя

The User object contains the following schema:
[inline-code-attrs-start title = 'Объект пользователя'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Обязательно. Максимум 1k символов. **/
    id: string;
    /** Обязательно. Максимум 1k символов. Примечание: должно быть уникальным. **/
    email: string;
    /** Обязательно. Максимум 1k символов. Примечание: username не может быть email. Не обязательно уникален. **/
    username: string;
    /** Необязательно. Максимум 3k символов для URL. По умолчанию берётся с gravatar на основе email. Поддерживает 64-битные закодированные изображения, в этом случае лимит — 50k символов. **/ 
    avatar?: string;
    /** Необязательно. По умолчанию false. **/
    optedInNotifications?: boolean;
    /** Необязательно. По умолчанию false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Необязательно. Максимум 100 символов. Эта надпись будет показана рядом с их именем. По умолчанию — Administrator/Moderator, когда применимо. **/
    displayLabel?: string;
    /** Необязательно. Максимум 500 символов. Будет показано вместо username. **/
    displayName?: string;
    /** Необязательно. Максимум 2k символов. Имя пользователя будет ссылаться на этот URL. **/
    websiteUrl?: string;
    /** Необязательно. До 100 групп на пользователя. ID группы не может быть длиннее 50 символов. **/
    groupIds?: string[];
    /** Необязательно. Помечает пользователя как администратора. **/
    isAdmin?: boolean;
    /** Необязательно. Помечает пользователя как модератора. **/
    isModerator?: boolean;
    /** Необязательно, по умолчанию true. Установите в false, чтобы включить вкладку "activity" в профиле пользователя. **/
    isProfileActivityPrivate?: boolean;
    /** Необязательно, по умолчанию false. Установите в true, чтобы отключить комментарии в профиле. **/
    isProfileCommentsPrivate?: boolean;
    /** Необязательно, по умолчанию false. Установите в true, чтобы отключить отправку личных сообщений этому пользователю. **/
    isProfileDMDisabled?: boolean;
}
[inline-code-end]

#### Модераторы и администраторы

Для админов и модераторов передайте соответствующие флаги `isAdmin` или `isModerator` в объекте `SSOUser`.

#### Уведомления

Чтобы включить или отключить уведомления, установите значение `optedInNotifications` в `true` или `false` соответственно. При первом загрузке страницы пользователем с этим значением в SSO-пейлоаде его настройки уведомлений будут обновлены.

Кроме того, если вы хотите, чтобы пользователи получали email-уведомления о событиях на страницах, на которые они подписаны (в отличие от только внутренних уведомлений в приложении), установите `optedInSubscriptionNotifications` в `true`.

#### VIP-пользователи и специальные метки

Вы можете отображать специальную метку рядом с именем пользователя, используя необязательное поле "displayLabel".

#### Неаутентифицированные пользователи

Чтобы представить неаутентифицированного пользователя, просто не заполняйте userDataJSONBase64, verificationHash или timestamp. Предоставьте loginURL.

Эти пользователи не смогут комментировать; им будет показано сообщение о входе (сообщение, ссылка или кнопка, в зависимости от конфигурации).

#### Прямые примеры сериализации и хеширования данных пользователя

Больше деталей и примеров можно найти <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">здесь</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">здесь</a> (java) и <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">здесь</a> (php).

Мы понимаем, что любая интеграция может быть сложным и болезненным процессом. Не стесняйтесь обращаться к вашему представителю или использовать <a href="https://fastcomments.com/auth/my-account/help" target="_blank">страницу поддержки</a>.