---
[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO использует шифрование HMAC-SHA256 как механизм реализации SSO. Сначала мы рассмотрим общую архитектуру, приведём примеры и подробные шаги.

Также имеется документация по миграции с других провайдеров с похожими SSO-механизмами и по отличиям.

Поток выглядит следующим образом:

<div class="screenshot white-bg">
    <div class="title">Процесс Secure SSO</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Secure SSO Diagram" />
</div>

Поскольку Secure SSO требует full-stack разработки, полнофункциональные примеры кода на Java/Spring, NodeJS/Express и чистом PHP в настоящее время <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">на GitHub</a>.

Хотя в примере NodeJS мы используем ExpressJS, а в примере Java — Spring, для реализации FastComments SSO в этих средах не требуются фреймворки/библиотеки — работают нативные криптографические пакеты.

Вам не нужно писать новые API-эндпоинты для FastComments SSO. Просто зашифруйте информацию о пользователе с помощью вашего секретного ключа и передайте полезную нагрузку в виджет комментариев.

#### Get Your API Secret Key

Ваш секретный ключ API можно получить на <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">этой странице</a>. Вы также можете найти эту страницу, перейдя в My Account, щёлкнув плитку API/SSO и затем нажав "Get API Secret Key".

#### Comment Widget Parameters

Документация высокого уровня по API для виджета комментариев доступна <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">здесь</a>.

Давайте подробнее рассмотрим, что означают эти параметры.

Виджет комментариев принимает объект конфигурации — вы уже передаёте его, если используете FastComments для передачи вашего идентификатора клиента (tenantId).

Чтобы включить SSO, передайте новый объект "sso", который должен иметь следующие параметры. Значения должны генерироваться на стороне сервера.

- userDataJSONBase64: Данные пользователя в формате JSON, которые затем кодируются в Base64.
- verificationHash: HMAC-SHA256 хеш, созданный из UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Временная метка эпохи (Epoch), в **миллисекундах**. Не может быть из будущего и не более двух дней в прошлом.
- loginURL: URL, который виджет комментариев может показать для входа пользователя.
- logoutURL: URL, который виджет комментариев может показать для выхода пользователя.
- loginCallback: Когда предоставлен вместо loginURL — функция, которую виджет комментариев вызовет при нажатии на кнопку входа.
- logoutCallback: Когда предоставлен вместо logoutURL — функция, которую виджет комментариев вызовет при нажатии на кнопку выхода.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### The User Object

The User object contains the following schema:
[inline-code-attrs-start title = 'Объект пользователя'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Обязательно. Максимум 1k символов. **/
    id: string;
    /** Обязательно. Максимум 1k символов. Примечание: Должно быть уникальным. **/
    email: string;
    /** Обязательно. Максимум 1k символов. Примечание: имя пользователя не может быть адресом электронной почты. Не обязательно быть уникальным. **/
    username: string;
    /** Необязательно. Максимум 3k символов для URL. По умолчанию берётся из gravatar на основе email. Поддерживает изображения, закодированные в base64, в этом случае лимит — 50k символов. **/ 
    avatar?: string;
    /** Необязательно. По умолчанию false. **/
    optedInNotifications?: boolean;
    /** Необязательно. По умолчанию false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Необязательно. Максимум 100 символов. Эта метка будет показана рядом с их именем. По умолчанию — Administrator/Moderator при необходимости. **/
    displayLabel?: string;
    /** Необязательно. Максимум 500 символов. Это будет показано вместо username. **/
    displayName?: string;
    /** Необязательно. Максимум 2k символов. Имя пользователя будет ссылаться на это. **/
    websiteUrl?: string;
    /** Необязательно. До 100 групп на пользователя. Идентификатор группы не может быть длиннее 50 символов. **/
    groupIds?: string[];
    /** Необязательно. Обозначает пользователя как администратора. **/
    isAdmin?: boolean;
    /** Необязательно. Обозначает пользователя как модератора. **/
    isModerator?: boolean;
    /** Необязательно, по умолчанию true. Установите в false, чтобы включить вкладку "activity" в профиле пользователя. **/
    isProfileActivityPrivate?: boolean;
    /** Необязательно, по умолчанию false. Установите в true, чтобы отключить комментарии в профиле. **/
    isProfileCommentsPrivate?: boolean;
    /** Необязательно, по умолчанию false. Установите в true, чтобы отключить личные сообщения этому пользователю. **/
    isProfileDMDisabled?: boolean;
    /** Необязательная конфигурация значков пользователя. **/
    badgeConfig?: {
        /** Массив глобальных идентификаторов значков для назначения. Ограничение — 30 значков. Порядок сохраняется. **/
        badgeIds: string[];
        /** Массив идентификаторов значков, привязанных к текущей странице (urlId). Показываются только на указанной странице. **/
        pageBadgeIds?: string[];
        /** Если true, заменяет отображаемые значки. Глобальные и привязанные к странице значки переопределяются независимо. **/
        override?: boolean;
        /** Если true, обновляет свойства отображения значков из конфигурации tenant. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Moderators and Administrators

Для администраторов и модераторов передайте соответствующие флаги `isAdmin` или `isModerator` в объекте `SSOUser`.

#### Notifications

Чтобы включить или отключить уведомления, установите значение `optedInNotifications` в `true` или `false` соответственно. При первом загрузке пользователем страницы с этим значением в SSO-пейлоуде его настройки уведомлений будут обновлены.

Дополнительно, если вы хотите, чтобы пользователи получали уведомления по электронной почте о действиях на страницах, на которые они подписаны (в отличие от только внутриигровых/встроенных уведомлений), установите `optedInSubscriptionNotifications` в `true`.

#### VIP Users & Special Labels

Вы можете отображать специальную метку рядом с именем пользователя, используя необязательное поле "displayLabel".

#### Unauthenticated users

Чтобы представить неаутентифицированного пользователя, просто не заполняйте userDataJSONBase64, verificationHash или timestamp. Предоставьте loginURL.

Эти пользователи не смогут оставлять комментарии; вместо этого им будет показано сообщение о входе (сообщение, ссылка или кнопка, в зависимости от конфигурации).

#### Direct Examples for Serializing and Hashing User Data

Больше деталей и примеров см. <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">здесь</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">здесь</a> (java) и <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">здесь</a> (php).

Мы понимаем, что любая интеграция может быть сложным и болезненным процессом. Не стесняйтесь связаться со своим представителем или воспользоваться <a href="https://fastcomments.com/auth/my-account/help" target="_blank">страницей поддержки</a>.

---