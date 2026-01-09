[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO използва HMAC-SHA256 криптиране като механизъм за реализиране на SSO. Първо ще разгледаме цялостната архитектура, ще дадем примери и подробни стъпки.

Има и документация относно мигриране от други доставчици с подобни SSO механизми и разликите.

Потокът изглежда по следния начин:

<div class="screenshot white-bg">
    <div class="title">Поток на Secure SSO</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Диаграма на Secure SSO" />
</div>

Тъй като Secure SSO включва full-stack разработка, пълни работещи примери на код в Java/Spring, NodeJS/Express и чист PHP в момента са <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">в GitHub</a>.

Въпреки че използваме ExpressJS в примера за NodeJS и Spring в Java примера, в тези среди не са необходими допълнителни фреймуърки/библиотеки, за да се реализира FastComments SSO - нативните крипто пакети работят.

Не е нужно да създавате никакви нови API endpoints за FastComments SSO. Просто криптирайте информацията за потребителя с вашия секретен ключ и предайте payload-а на коментарния widget.

#### Get Your API Secret Key

Вашият API Secret може да бъде извлечен от <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">тази страница</a>. Можете да намерите тази страница и като отидете в My Account, кликнете върху плочката API/SSO и след това кликнете "Get API Secret Key".

#### Comment Widget Parameters

Документация на високо ниво за API на коментарния widget може да бъде намерена <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">тук</a>.

Нека разгледаме по-подробно какво означават тези параметри.

Коментарният widget приема обект за конфигурация - вие вече подавате това, ако използвате FastComments, за да предадете своя customer id (наречен tenantId).

За да активирате SSO, подайте нов обект "sso", който трябва да има следните параметри. Стойностите трябва да бъдат генерирани от сървърната страна.

- userDataJSONBase64: Данните за потребителя в JSON формат, които след това са кодирани в Base64.
- verificationHash: HMAC-SHA256 хеш, създаден от UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Епохален времеви печат, в **милисекунди**. Не трябва да е в бъдещето или повече от два дни в миналото.
- loginURL: URL, който коментарният widget може да покаже за вписване на потребителя.
- logoutURL: URL, който коментарният widget може да покаже за излизане на потребителя.
- loginCallback: Когато се предостави вместо login URL, функция която коментарният widget ще извика при натискане на бутона за вход.
- logoutCallback: Когато се предостави вместо logout URL, функция която коментарният widget ще извика при натискане на бутона за излизане.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### Потребителски обект

[inline-code-attrs-start title = 'Потребителски обект'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Задължително. Максимум 1k символа. **/
    id: string;
    /** Задължително. Максимум 1k символа. Забележка: Трябва да е уникален. **/
    email: string;
    /** Задължително. Максимум 1k символа. Забележка: Потребителското име не може да бъде имейл. Не е необходимо да е уникално. **/
    username: string;
    /** По избор. Максимум 3k символа за URL адреси. По подразбиране идва от gravatar базиран на имейл. Поддържа Base64-кодирани изображения, в този случай ограничението е 50k символа. **/ 
    avatar?: string;
    /** По избор. По подразбиране false. **/
    optedInNotifications?: boolean;
    /** По избор. По подразбиране false. **/
    optedInSubscriptionNotifications?: boolean;
    /** По избор. Максимум 100 символа. Този етикет ще се покаже до името им. По подразбиране е Administrator/Moderator, когато е приложимо. **/
    displayLabel?: string;
    /** По избор. Максимум 500 символа. Това ще се покаже вместо потребителското име. **/
    displayName?: string;
    /** По избор. Максимум 2k символа. Името на потребителя ще води към този линк. **/
    websiteUrl?: string;
    /** По избор. До 100 групи на потребител. Идентификатор на група не може да бъде по-дълъг от 50 символа. **/
    groupIds?: string[];
    /** По избор. Означава потребителя като администратор. **/
    isAdmin?: boolean;
    /** По избор. Означава потребителя като модератор. **/
    isModerator?: boolean;
    /** По избор, по подразбиране true. Задайте на false, за да разрешите раздела "activity" в профила на потребителя. **/
    isProfileActivityPrivate?: boolean;
    /** По избор, по подразбиране false. Задайте на true, за да деактивирате коментарите в профила. **/
    isProfileCommentsPrivate?: boolean;
    /** По избор, по подразбиране false. Задайте на true, за да деактивирате директните съобщения до този потребител. **/
    isProfileDMDisabled?: boolean;
}
[inline-code-end]

#### Moderators and Administrators

За админи и модератори, подайте съответните флагове `isAdmin` или `isModerator` в обекта `SSOUser`.

#### Notifications

За да включите или изключите известията, задайте стойността на `optedInNotifications` на `true` или `false` съответно. Първият път когато потребителят зареди страницата с тази стойност в SSO payload-а, настройките му за известия ще бъдат обновени.

Освен това, ако искате потребителите да получават имейли за известия за активност на страници, на които са абонирани (а не само известия в приложението), задайте `optedInSubscriptionNotifications` на `true`.

#### VIP Users & Special Labels

Можете да покажете специален етикет до името на потребителя, като използвате опционалното поле "displayLabel".

#### Unauthenticated users

За да представите непотребител (unauthenticated user), просто не попълвайте userDataJSONBase64, verificationHash или timestamp. Осигурете loginURL.

Тези потребители няма да могат да коментират и вместо това ще им бъде показано съобщение за вход (съобщение, линк или бутон, в зависимост от конфигурацията).

#### Direct Examples for Serializing and Hashing User Data

Повече детайли и примери могат да бъдат намерени <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">тук</a> (js), <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">тук</a> (java) и <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">тук</a> (php).

Разбираме, че всяка интеграция може да бъде сложен и болезнен процес. Не се колебайте да се свържете с вашия представител или да използвате <a href="https://fastcomments.com/auth/my-account/help" target="_blank">страницата за поддръжка</a>.