[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO користи HMAC-SHA256 енкрипцију као механизам за имплементацију SSO. Прво ћемо проћи кроз општу архитектуру, дати примере и детаљне кораке.

Постоји и документација у вези са миграцијом са других провајдера који имају сличне SSO механизме, и разликама.

The flow looks like this:

<div class="screenshot white-bg">
    <div class="title">Secure SSO Flow</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Secure SSO Diagram" />
</div>

Since Secure SSO involves full-stack development, full working code examples in Java/Spring, NodeJS/Express, and vanilla PHP are currently <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">на GitHub-у</a>.

Although we use ExpressJS in the NodeJS example and Spring in the Java example there are no frameworks/libraries required in these run-times to implement FastComments SSO - the native crypto packages work.

You don't have to write any new API endpoints with FastComments SSO. Simply encrypt the user's info using your secret key and pass the payload to the comment widget.

#### Get Your API Secret Key

Your API Secret can be retrieved from <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">this page</a>. You can find this page also by going to My Account, clicking the API/SSO tile, and then clicking "Get API Secret Key".

#### Comment Widget Parameters

High-level API documentation for the comment widget can be found <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">here</a>.

Let's go into more detail of what these parameters mean.

The comment widget takes a configuration object - you already pass this if you're using FastComments to pass your customer id (called tenantId).

To enable SSO, pass a new "sso" object, which must have the following parameters. The values should be generated server side.

- userDataJSONBase64: Подаци корисника у JSON формату, који се затим Base64 енкодирају.
- verificationHash: HMAC-SHA256 хеш креиран из UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Епохни временски ознака, у **милисекундама**. Не сме бити у будућности, или више од два дана у прошлости.
- loginURL: URL који коментар видгет може приказати за пријављивање корисника.
- logoutURL: URL који коментар видгет може приказати за одјављивање корисника.
- loginCallback: Када је обезбеђена уместо login URL-а, функција коју ће коментар видгет позвати при клику на дугме за пријављивање.
- logoutCallback: Када је обезбеђена уместо logout URL-а, функција коју ће коментар видгет позвати при клику на дугме за одјављивање.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### Објекат корисника

[inline-code-attrs-start title = 'Објекат корисника'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Обавезно. 1k Characters Max. **/
    id: string;
    /** Обавезно. 1k Characters Max. Напомена: Мора бити јединствено. **/
    email: string;
    /** Обавезно. 1k Characters Max. Напомена: Корисничко име не може бити е-пошта. Не мора бити јединствено. **/
    username: string;
    /** Опционо. 3k Characters Max for URLs. Default is from gravatar based on email. Supports 64 encoded images, in which case the limit is 50k characters. **/ 
    avatar?: string;
    /** Опционо. Подразумевано false. **/
    optedInNotifications?: boolean;
    /** Опционо. Подразумевано false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Опционо. 100 Characters Max. Ова ознака ће бити приказана поред њиховог имена. Подразумевано је Администратор/Модератор када је применљиво. **/
    displayLabel?: string;
    /** Опционо. 500 Characters Max. Ово ће бити приказано уместо корисничког имена. **/
    displayName?: string;
    /** Опционо. 2k Characters Max. Име корисника ће водити на ово. **/
    websiteUrl?: string;
    /** Опционо. До 100 група по кориснику. A group id may not be longer than 50 characters. **/
    groupIds?: string[];
    /** Опционо. Ознака која означава корисника као администратора. **/
    isAdmin?: boolean;
    /** Опционо. Ознака која означава корисника као модератора. **/
    isModerator?: boolean;
    /** Опционо, подразумевано true. Поставите на false да бисте омогућили картицу "activity" у профилу корисника. **/
    isProfileActivityPrivate?: boolean;
    /** Опционо, подразумевано false. Поставите на true да онемогућите коментаре на профилу. **/
    isProfileCommentsPrivate?: boolean;
    /** Опционо, подразумевано false. Поставите на true да онемогућите директне поруке овом кориснику. **/
    isProfileDMDisabled?: boolean;
}
[inline-code-end]

#### Moderators and Administrators

For admins and moderators, pass the respective `isAdmin` or `isModerator` flags in the `SSOUser` object.

#### Notifications

To enable or disable notifications, set the value of `optedInNotifications` to `true` or `false` respectively. The first time the user loads the page with this value in the SSO payload, their notification settings will be updated.

Additionally, if you want users to receive notification emails for activity on pages they subscribed to (as opposed to just in-app notifications), then set `optedInSubscriptionNotifications` to `true`.

#### VIP Users & Special Labels

You can display a special label next to the user's name by using the optional "displayLabel" field.

#### Unauthenticated users

To represent an unauthenticated user, simply do not populate userDataJSONBase64, verificationHash, or timestamp. Provide a loginURL.

These users will not be able to comment, and instead will be presented with a login message (message, link, or button, depending on configuration).

#### Direct Examples for Serializing and Hashing User Data

More details as an examples <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">here</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">here</a> (java) and <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">here</a> (php).

We understand that any integration can be a complicated and painful process. Don't hesitate to reach out to your representative or use the <a href="https://fastcomments.com/auth/my-account/help" target="_blank">support page</a>.