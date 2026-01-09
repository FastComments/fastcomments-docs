[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO користи HMAC-SHA256 енкрипцију као механизам за имплементацију SSO. Прво ћемо проћи кроз општу архитектуру, пружити примере и детаљне кораке.

Постоји и нека документација у вези миграције са других провајдера са сличним SSO механизмима, и разлика.

Ток изгледа овако:

<div class="screenshot white-bg">
    <div class="title">Ток сигурног SSO</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Дијаграм сигурног SSO" />
</div>

Пошто Secure SSO подразумева full-stack развој, потпуно функционални примери кода у Java/Spring, NodeJS/Express и vanilla PHP су тренутно <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">на GitHub-у</a>.

Иако у NodeJS примеру користимо ExpressJS а у Java примеру Spring, у овим runtime окружењима нису потребне додатне библиотеке/фрејмворци да бисте имплементирали FastComments SSO — нативни crypto пакети су довољни.

Не морате да правите нове API endpoint-ове за FastComments SSO. Једноставно енкриптујте информације о кориснику користећи ваш тајни кључ и проследите payload у comment widget.

#### Добијте ваш API тајни кључ

Ваш API тајни кључ можете преузети са <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">ове странице</a>. Такође можете доћи до ове странице тако што ћете отићи на My Account, кликнути на API/SSO плочицу, а затим кликнути "Get API Secret Key".

#### Параметри видгета за коментаре

Високо-ниво API документације за comment widget можете пронаћи <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">овде</a>.

Хајде да детаљније размотримо шта ти параметри значе.

Comment widget прима конфигурациони објекат — већ прослеђујете овај објекат ако користите FastComments да бисте проследили ваш tenantId (зване customer id).

Да бисте омогућили SSO, проследите нови објекат "sso", који мора имати следеће параметре. Вредности треба да буду генерисане на серверској страни.

- userDataJSONBase64: Подаци корисника у JSON формату, који се затим кодују у Base64.
- verificationHash: HMAC-SHA256 хеш направљен од UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Epoch timestamp у **милисекундама**. Не сме бити у будућности, нити више од два дана у прошлости.
- loginURL: URL који comment widget може приказати за пријављивање корисника.
- logoutURL: URL који comment widget може приказати за одјављивање корисника.
- loginCallback: Када је обезбеђено уместо login URL-а, функција коју ће comment widget позвати када се кликне дугме за пријаву.
- logoutCallback: Када је обезбеђено уместо logout URL-а, функција коју ће comment widget позвати када се кликне дугме за одјаву.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### Кориснички објекат

The User Object:
[inline-code-attrs-start title = 'Кориснички објекат'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Обавезно. Максимум 1k карактера. **/
    id: string;
    /** Обавезно. Максимум 1k карактера. Напомена: Мора бити јединствено. **/
    email: string;
    /** Обавезно. Максимум 1k карактера. Напомена: Корисничко име не може бити имејл. Не мора бити јединствено. **/
    username: string;
    /** Опционо. Максимум 3k карактера за URL-ове. Подразумевано је са gravatar-а на основу имејла. Подржава 64 кодиране слике, у ком случају је лимит 50k карактера. **/ 
    avatar?: string;
    /** Опционо. Подразумевано false. **/
    optedInNotifications?: boolean;
    /** Опционо. Подразумевано false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Опционо. Максимум 100 карактера. Ова ознака ће бити приказана поред њиховог имена. Подразумевано је Administrator/Moderator када је применљиво. **/
    displayLabel?: string;
    /** Опционо. Максимум 500 карактера. Ово ће бити приказано уместо корисничког имена. **/
    displayName?: string;
    /** Опционо. Максимум 2k карактера. Име корисника ће водити на ово. **/
    websiteUrl?: string;
    /** Опционо. До 100 група по кориснику. ID групе не може бити дужи од 50 карактера. **/
    groupIds?: string[];
    /** Опционо. Означава корисника као администратора. **/
    isAdmin?: boolean;
    /** Опционо. Означава корисника као модератора. **/
    isModerator?: boolean;
    /** Опционо, подразумевано true. Поставите на false да омогућите "activity" таб у корисничком профилу. **/
    isProfileActivityPrivate?: boolean;
    /** Опционо, подразумевано false. Поставите на true да онемогућите коментаре на профилу. **/
    isProfileCommentsPrivate?: boolean;
    /** Опционо, подразумевано false. Поставите на true да онемогућите слање директних порука овом кориснику. **/
    isProfileDMDisabled?: boolean;
}
[inline-code-end]

#### Модератори и администратори

За администраторе и модераторе, проследите одговарајуће `isAdmin` или `isModerator` ознаке у објекту `SSOUser`.

#### Обавештења

Да бисте омогућили или онемогућили обавештења, подесите вредност `optedInNotifications` на `true` или `false` респективно. Први пут када корисник учита страницу са овом вредношћу у SSO payload-у, њихова подешавања обавештења ће бити ажурирана.

Додатно, ако желите да корисници примају електронске поруке обавештења за активност на страницама на које су се претплатили (за разлику од само апликационих обавештења), подесите `optedInSubscriptionNotifications` на `true`.

#### VIP корисници и посебне ознаке

Можете приказати посебну ознаку поред имена корисника коришћењем опционог поља "displayLabel".

#### Неаутентификовани корисници

Да бисте представили неаутентификованог корисника, једноставно не попуњавајте userDataJSONBase64, verificationHash или timestamp. Обезбедите loginURL.

Ови корисници неће моћи да коментаришу, већ ће им бити приказана порука за пријаву (порука, линк или дугме, у зависности од конфигурације).

#### Директни примери за серијализацију и хеширање података корисника

Више детаља и примера можете наћи <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">овде</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">овде</a> (java) и <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">овде</a> (php).

Разумемо да било која интеграција може бити компликован и болан процес. Не оклевајте да контактирате вашег представника или да користите <a href="https://fastcomments.com/auth/my-account/help" target="_blank">страницу за подршку</a>.