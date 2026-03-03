[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO користи HMAC-SHA256 енкрипцију као механизам за реализовање SSO. Прво ћемо проћи кроз општу архитектуру, дати примере и детаљне кораке.

Постоји и документација у вези миграције са других провајдера са сличним SSO механизмима, и разлике.

Ток изгледа овако:

<div class="screenshot white-bg">
    <div class="title">Secure SSO ток</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Дијаграм Secure SSO" />
</div>

Пошто Secure SSO подразумева full-stack development, потпуно радни примери кода у Java/Spring, NodeJS/Express и vanilla PHP су тренутно <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">на GitHub-у</a>.

Иако користимо ExpressJS у NodeJS примеру и Spring у Java примеру, у овим runtime-овима нису потребни додатни фрејмворци/библиотеке да бисте имплементирали FastComments SSO — нативни crypto пакети су довољни.

Не морате да пишете нове API ендпоинте са FastComments SSO. Једноставно енкриптујте информације о кориснику користећи ваш тајни кључ и проследите payload видгету за коментаре.

#### Добијте ваш API тајни кључ

Ваш API Secret можете преузети са <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">ове странице</a>. Такође можете доћи до ове странице тако што ћете отићи на My Account, кликнути на API/SSO плочицу, а затим кликнути на "Get API Secret Key".

#### Параметри видгета за коментаре

Документација високог нивоа за API видгета за коментаре може се наћи <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">овде</a>.

Хајде да детаљније објаснимо шта ти параметри значе.

Видгет за коментаре прима конфигурациони објекат — већ га прослеђујете ако користите FastComments за прослеђивање вашег идентификатора купца (назван tenantId).

Да бисте омогућили SSO, проследите нови "sso" објекат, који мора да садржи следеће параметре. Вредности треба да буду генерисане на серверу.

- userDataJSONBase64: Подaци корисника у JSON формату, који се затим Base64 кодирају.
- verificationHash: HMAC-SHA256 хеш креиран из UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Епохални timestamp, у **милисекундама**. Не сме бити у будућности, нити старији од два дана.
- loginURL: URL који видгет за коментаре може показати за пријављивање корисника.
- logoutURL: URL који видгет за коментаре може показати за одјављивање корисника.
- loginCallback: Када је обезбеђен уместо login URL-а, функција коју ће видгет за коментаре позвати приликом клика на дугме за пријаву.
- logoutCallback: Када је обезбеђен уместо logout URL-а, функција коју ће видгет за коментаре позвати приликом клика на дугме за одјаву.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### Кориснички објекат

The User Object:
[inline-code-attrs-start title = 'Кориснички објекат'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Обавезно. Максимум 1k карактера. **/
    id: string;
    /** Обавезно. Максимум 1k карактера. Напомена: Мора бити јединствен. **/
    email: string;
    /** Обавезно. Максимум 1k карактера. Напомена: Корисничко име не може бити e-mail. Не мора бити јединствено. **/
    username: string;
    /** Опционо. Максимално 3k карактера за URL-ове. Подразумевано долази са gravatar-а на основу e-mail-а. Подржава base64-енкодиране слике, у ком случају лимит је 50k карактера. **/ 
    avatar?: string;
    /** Опционо. Подразумевано false. **/
    optedInNotifications?: boolean;
    /** Опционо. Подразумевано false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Опционо. Максимум 100 карактера. Ова ознака ће бити приказана поред њиховог имена. Подразумевано је Administrator/Moderator када је применљиво. **/
    displayLabel?: string;
    /** Опционо. Максимум 500 карактера. Ово ће бити приказано уместо username-а. **/
    displayName?: string;
    /** Опционо. Максимум 2k карактера. Име корисника ће водити на ово. **/
    websiteUrl?: string;
    /** Опционо. До 100 група по кориснику. ИД групе не сме бити дужи од 50 карактера. **/
    groupIds?: string[];
    /** Опционо. Означава корисника као администратора. **/
    isAdmin?: boolean;
    /** Опционо. Означава корисника као модератора. **/
    isModerator?: boolean;
    /** Опционо, подразумевано true. Поставите на false да бисте омогућили таб "activity" у профилу корисника. **/
    isProfileActivityPrivate?: boolean;
    /** Опционо, подразумевано false. Поставите на true да онемогућите коментаре на профилу. **/
    isProfileCommentsPrivate?: boolean;
    /** Опционо, подразумевано false. Поставите на true да онемогућите директне поруке овом кориснику. **/
    isProfileDMDisabled?: boolean;
    /** Опциона конфигурација за значке корисника. **/
    badgeConfig?: {
        /** Низ глобалних ID-ева значки које треба доделити. Ограничено на 30 значки. Редослед се поштује. **/
        badgeIds: string[];
        /** Низ ID-ева значки ограничених на тренутну страницу (urlId). Приказују се само на додељеној страници. **/
        pageBadgeIds?: string[];
        /** Ако је true, замењује постојеће приказане значке. Глобалне и странице-ограничене значке се понаособ преуписују. **/
        override?: boolean;
        /** Ако је true, ажурира приказ значки из конфигурације tenant-а. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Модератори и администратори

За администраторе и модераторе, проследите одговарајуће флагове `isAdmin` или `isModerator` у `SSOUser` објекту.

#### Обавештења

Да бисте омогућили или онемогућили обавештења, поставите вредност `optedInNotifications` на `true` или `false` респективно. Први пут када корисник учита страницу са овом вредношћу у SSO payload-у, њихова подешавања обавештења ће бити ажурирана.

Додатно, ако желите да корисници примају е-маил обавештења за активност на страницама на које су претплаћени (уместо само in-app обавештења), поставите `optedInSubscriptionNotifications` на `true`.

#### VIP корисници и посебне ознаке

Можете приказати посебну ознаку поред имена корисника користећи опционо поље "displayLabel".

#### Неаутентификовани корисници

Да бисте представили неаутентификованог корисника, једноставно не попуните userDataJSONBase64, verificationHash или timestamp. Обезбедите loginURL.

Ови корисници неће моћи да коментаришу, уместо тога ће им бити приказана порука за пријаву (порука, линк или дугме, у зависности од конфигурације).

#### Примери за серијализацију и хеширање података корисника

Више детаља и примера можете наћи <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">овде</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">овде</a> (java) и <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">овде</a> (php).

Разумемо да било која интеграција може бити компликован и тежак процес. Не оклевајте да контактирате вашег представника или користите <a href="https://fastcomments.com/auth/my-account/help" target="_blank">страницу за подршку</a>.

---