[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO користи HMAC-SHA256 енкрипцију као механизам за имплементацију SSO. Прво ћемо проћи кроз општу архитектуру, дати примјере и детаљне кораке.

Постоји и документација у вези миграције са других провајдера који имају сличне SSO механизме, и разликаима.

Ток изгледа овако:

<div class="screenshot white-bg">
    <div class="title">Ток сигурног SSO</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Дијаграм сигурног SSO" />
</div>

Пошто Secure SSO укључује full-stack развој, пуни радни примјери кода у Java/Spring, NodeJS/Express, и ванила PHP су тренутно <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">на GitHub-у</a>.

Иако користимо ExpressJS у NodeJS примјеру и Spring у Java примјеру, нема потребе за додатним фрејмворцима/библиотекама у овим runtime окружењима да бисте имплементирали FastComments SSO - нативни крипто пакети раде.

Не морате писати нове API ентропоинте са FastComments SSO. Једноставно енкриптујте информације о кориснику користећи ваш тајни кључ и прослиједите payload видгету за коментаре.

#### Добијте ваш API тајни кључ

Ваш API тајни кључ можете преузети са <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">ове странице</a>. Ову страницу такођер можете пронаћи тако што ћете отићи на My Account, кликнути на API/SSO плочицу, а затим кликнути "Get API Secret Key".

#### Параметри видгета коментара

Документација високог нивоа за API видгета коментара може се наћи <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">овдје</a>.

Хајде да детаљније погледамо шта ти параметри значе.

Видгет коментара узима објекат конфигурације - већ га прослеђујете ако користите FastComments да пошаљете ваш customer id (назван tenantId).

Да бисте омогућили SSO, проследите нови "sso" објекат, који мора имати сљедеће параметре. Вриједности треба да буду генерисане на страни сервера.

- userDataJSONBase64: Подаци корисника у JSON формату, који су потом Base64 кодирани.
- verificationHash: HMAC-SHA256 хеш направљен од UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Епоха тајмстамп, у **милисекундама**. Не смије бити у будућности, нити старији више од два дана.
- loginURL: URL који видгет коментара може приказати како би пријавио корисника.
- logoutURL: URL који видгет коментара може приказати како би одјавио корисника.
- loginCallback: Када је обезбијеђен уместо login URL-а, функција коју ће видгет коментара позвати када се кликне на дугме за пријаву.
- logoutCallback: Када је обезбијеђен уместо logout URL-а, функција коју ће видгет коментара позвати када се кликне на дугме за одјаву.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### Објекат корисника

Објекат корисника садржи сљедећу шему:
[inline-code-attrs-start title = 'Објекат корисника'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Обавезно. Максимум 1k знакова. **/
    id: string;
    /** Обавезно. Максимум 1k знакова. Напомена: Мора бити јединствено. **/
    email: string;
    /** Обавезно. Максимум 1k знакова. Напомена: Корисничко име не може бити имејл. Не мора бити јединствено. **/
    username: string;
    /** Опционо. Максимум 3k знакова за URL-ове. Подразумевано се узима из gravatar-а на основу имејла. Подржава Base64 кодиране слике, у којем случају лимит је 50k знакова. **/ 
    avatar?: string;
    /** Опционо. Подразумевано false. **/
    optedInNotifications?: boolean;
    /** Опционо. Подразумевано false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Опционо. Максимум 100 карактера. Ова ознака ће бити приказана поред њиховог имена. Подразумевано је Administrator/Moderator када је примјењиво. **/
    displayLabel?: string;
    /** Опционо. Максимум 500 карактера. Ово ће бити приказано умјесто корисничког имена. **/
    displayName?: string;
    /** Опционо. Максимум 2k знакова. Име корисника ће водити на ово. **/
    websiteUrl?: string;
    /** Опционо. До 100 група по кориснику. ИД групе не смије бити дужи од 50 карактера. **/
    groupIds?: string[];
    /** Опционо. Означава корисника као администратора. **/
    isAdmin?: boolean;
    /** Опционо. Означава корисника као модератора. **/
    isModerator?: boolean;
    /** Опционо, подразумевано true. Поставите на false да омогућите таб "activity" у профилу корисника. **/
    isProfileActivityPrivate?: boolean;
    /** Опционо, подразумевано false. Поставите на true да онемогућите коментаре на профилу. **/
    isProfileCommentsPrivate?: boolean;
    /** Опционо, подразумевано false. Поставите на true да онемогућите директне поруке овог корисника. **/
    isProfileDMDisabled?: boolean;
}
[inline-code-end]

#### Модератори и администратори

За администраторе и модераторе, проследите одговарајуће `isAdmin` или `isModerator` флаге у објекту `SSOUser`.

#### Обавјештења

Да бисте омогућили или онемогућили обавјештења, поставите вриједност `optedInNotifications` на `true` или `false` респективно. Први пут када корисник учита страницу са овом вриједношћу у SSO payload-у, њихова подешавања обавјештења ће бити ажурирана.

Додатно, ако желите да корисници примају обавјештења путем е-поште за активност на страницама на које су се претплатили (за разлику од само апликацијских обавјештења), онда поставите `optedInSubscriptionNotifications` на `true`.

#### VIP корисници и посебне ознаке

Можете приказати посебну ознаку поред имена корисника коришћењем опционог поља "displayLabel".

#### Неаутентификовани корисници

Да бисте представили неаутентификованог корисника, једноставно не попуњавајте userDataJSONBase64, verificationHash, или timestamp. Обезбиједите loginURL.

Ови корисници неће моћи коментарисати, већ ће им бити приказана порука за пријаву (порука, линк, или дугме, у зависности од конфигурације).

#### Примјери за серијализацију и хеширање података корисника

Више детаља као примјери можете пронаћи <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">овдје</a> (js), <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">овдје</a> (java) и <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">овдје</a> (php).

Разумијемо да било која интеграција може бити компликован и болан процес. Не оклијевајте да контактирате вашег представника или користите <a href="https://fastcomments.com/auth/my-account/help" target="_blank">страницу за подршку</a>.