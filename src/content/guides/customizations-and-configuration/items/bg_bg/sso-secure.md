[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO използва HMAC-SHA256 криптиране като механизъм за реализиране на SSO. Първо ще разгледаме общата архитектура, ще предоставим примери и подробни стъпки.

Има също документация относно миграцията от други доставчици със сходни механизми за SSO и разликите.

Потокът изглежда така:

<div class="screenshot white-bg">
    <div class="title">Поток на Secure SSO</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Диаграма на Secure SSO" />
</div>

Тъй като Secure SSO включва разработка от край до край, пълни работещи примери на код на Java/Spring, NodeJS/Express и чист PHP в момента са <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">в GitHub</a>.

Въпреки че в примера за NodeJS използваме ExpressJS и в Java примера използваме Spring, не са необходими допълнителни фреймуърци/библиотеки в тези среди за реализиране на FastComments SSO — работят собствените крипто пакети на езика.

Не е нужно да пишете нови API крайни точки с FastComments SSO. Просто криптирайте информацията за потребителя с вашия таен ключ и подайте payload-а на коментарния уиджет.

#### Вземете своя API Secret Key

Вашият API Secret може да бъде получен от <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">тази страница</a>. Можете също да намерите тази страница като отидете в My Account, кликнете върху плочката API/SSO и след това кликнете "Get API Secret Key".

#### Параметри на коментарния уиджет

Документация на високо ниво за API на коментарния уиджет може да бъде намерена <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">тук</a>.

Нека разгледаме по-подробно какво означават тези параметри.

Коментарният уиджет приема обект за конфигурация — вече подавате това, ако използвате FastComments, за да подадете вашия customer id (наречен tenantId).

За да активирате SSO, подайте нов обект "sso", който трябва да има следните параметри. Стойностите трябва да се генерират от страна на сървъра.

- userDataJSONBase64: Данните на потребителя в JSON формат, които след това се кодират в Base64.
- verificationHash: HMAC-SHA256 хеш, създаден от UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Епохален timestamp, в **милисекунди**. Не трябва да е в бъдещето или да е с повече от два дни в миналото.
- loginURL: URL, който коментарният уиджет може да покаже за вписване на потребителя.
- logoutURL: URL, който коментарният уиджет може да покаже за излизане на потребителя.
- loginCallback: Когато се предостави вместо login URL, функция, която коментарният уиджет ще извика при клик върху бутона за вход.
- logoutCallback: Когато се предостави вместо logout URL, функция, която коментарният уиджет ще извика при клик върху бутона за изход.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### Обект на потребителя

Потребителският обект съдържа следната схема:
[inline-code-attrs-start title = 'Обект на потребителя'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Задължително. Максимум 1k знака. **/
    id: string;
    /** Задължително. Максимум 1k знака. Забележка: Трябва да е уникален. **/
    email: string;
    /** Задължително. Максимум 1k знака. Забележка: Потребителското име не може да бъде имейл. Не е необходимо да е уникално. **/
    username: string;
    /** По избор. Максимум 3k знака за URL-и. По подразбиране се използва от gravatar въз основа на имейла. Поддържа 64 кодирани изображения, в който случай лимитът е 50k знака. **/ 
    avatar?: string;
    /** По избор. По подразбиране false. **/
    optedInNotifications?: boolean;
    /** По избор. По подразбиране false. **/
    optedInSubscriptionNotifications?: boolean;
    /** По избор. Максимум 100 знака. Тази етикета ще бъде показана до тяхното име. По подразбиране е Администратор/Модератор когато е приложимо. **/
    displayLabel?: string;
    /** По избор. Максимум 500 знака. Това ще се показва вместо потребителското име. **/
    displayName?: string;
    /** По избор. Максимум 2k знака. Името на потребителя ще се свързва към това. **/
    websiteUrl?: string;
    /** По избор. До 100 групи на потребител. Идентификатор на група не може да е по-дълъг от 50 знака. **/
    groupIds?: string[];
    /** По избор. Означава потребителя като администратор. **/
    isAdmin?: boolean;
    /** По избор. Означава потребителя като модератор. **/
    isModerator?: boolean;
    /** По избор, по подразбиране true. Задайте на false, за да активирате раздела "activity" в профила на потребителя. **/
    isProfileActivityPrivate?: boolean;
    /** По избор, по подразбиране false. Задайте на true, за да деактивирате коментарите в профила. **/
    isProfileCommentsPrivate?: boolean;
    /** По избор, по подразбиране false. Задайте на true, за да деактивирате директните съобщения към този потребител. **/
    isProfileDMDisabled?: boolean;
    /** По изборна конфигурация за значки на потребителя. **/
    badgeConfig?: {
        /** Масив от глобални идентификатори на значки за присвояване. Ограничение до 30 значки. Подреждането се запазва. **/
        badgeIds: string[];
        /** Масив от идентификатори на значки, обвързани със текущата страница (urlId). Показват се само на присвоената страница. **/
        pageBadgeIds?: string[];
        /** Ако е true, заменя вече показваните значки. Глобалните и страницево обвързаните се презаписват независимо. **/
        override?: boolean;
        /** Ако е true, обновява свойствата за показване на значките от конфигурацията на tenant-а. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Moderators and Administrators

За администратори и модератори, подайте съответните флагове `isAdmin` или `isModerator` в обекта `SSOUser`.

#### Notifications

За да активирате или деактивирате известията, задайте стойността на `optedInNotifications` на `true` или `false` съответно. Първият път когато потребителят зареди страницата със тази стойност в SSO полезния товар, настройките му за известия ще бъдат обновени.

Освен това, ако искате потребителите да получават имейли за известия за активност на страници, на които са се абонирали (за разлика от само вътрешните известия в приложението), задайте `optedInSubscriptionNotifications` на `true`.

#### VIP Users & Special Labels

Можете да покажете специален етикет до името на потребителя чрез опционалното поле "displayLabel".

#### Unauthenticated users

За да представите неаутентифициран потребител, просто не попълвайте userDataJSONBase64, verificationHash или timestamp. Осигурете loginURL.

Тези потребители няма да могат да коментират и вместо това ще им се покаже съобщение за влизане (съобщение, линк или бутон, в зависимост от конфигурацията).

#### Direct Examples for Serializing and Hashing User Data

Повече детайли и примери могат да бъдат намерени <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">тук</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">тук</a> (java) и <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">тук</a> (php).

Разбираме, че всяка интеграция може да бъде сложен и болезнен процес. Не се колебайте да се свържете с вашия представител или да използвате <a href="https://fastcomments.com/auth/my-account/help" target="_blank">страницата за поддръжка</a>.

---