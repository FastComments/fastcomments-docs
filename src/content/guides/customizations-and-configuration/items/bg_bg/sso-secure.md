[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO използва HMAC-SHA256 криптиране като механизъм за имплементиране на SSO. Първо ще разгледаме общата архитектура, ще предоставим примери и подробни стъпки.

Има и документация относно мигриране от други доставчици с подобни SSO механизми, както и разликите.

Процесът изглежда така:

<div class="screenshot white-bg">
    <div class="title">Secure SSO Flow</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Диаграма на Secure SSO" />
</div>

Тъй като Secure SSO включва full-stack разработка, пълни работещи примери с код на Java/Spring, NodeJS/Express и чист PHP в момента са <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">в GitHub</a>.

Макар че използваме ExpressJS в примера за NodeJS и Spring в примера за Java, няма необходими framework-и/библиотеки в тези среди за имплементиране на FastComments SSO - вградените crypto пакети работят.

Не е нужно да пишете никакви нови API endpoints с FastComments SSO. Просто криптирайте информацията на потребителя с вашия секретен ключ и подайте payload-а на коментарния уиджет.

#### Получете вашия API Secret Key

Вашият API Secret може да бъде получен от <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">тази страница</a>. Можете да намерите тази страница и като отидете на My Account, кликнете върху плочката API/SSO и след това кликнете "Get API Secret Key".

#### Параметри на коментарния уиджет

Документация на високо ниво за коментарния уиджет може да бъде намерена <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">тук</a>.

Нека разгледаме по-подробно какво означават тези параметри.

Коментарният уиджет приема конфигурационен обект - вие вече го подавате, ако използвате FastComments, за да предадете вашия customer id (наречен tenantId).

За да активирате SSO, подайте нов обект "sso", който трябва да има следните параметри. Стойностите трябва да бъдат генерирани от сървъра.

- userDataJSONBase64: Данните на потребителя в JSON формат, които след това са Base64 кодирани.
- verificationHash: HMAC-SHA256 хеш, създаден от UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Epoch timestamp, в **милисекунди**. Не трябва да е в бъдещето или повече от два дни в миналото.
- loginURL: URL адрес, който коментарният уиджет може да покаже за влизане на потребителя.
- logoutURL: URL адрес, който коментарният уиджет може да покаже за излизане на потребителя.
- loginCallback: Когато е предоставен вместо login URL, функция, която коментарният уиджет ще извика при кликване върху бутона за влизане.
- logoutCallback: Когато е предоставен вместо logout URL, функция, която коментарният уиджет ще извика при кликване върху бутона за излизане.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Клиентски код за Secure SSO'; isFunctional = false; code-example-end]

#### Обектът User

Обектът User съдържа следната схема:
[inline-code-attrs-start title = 'Обектът User'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Задължително. Максимум 1k символа. **/
    id: string;
    /** Задължително. Максимум 1k символа. Забележка: Трябва да е уникален. **/
    email: string;
    /** Задължително. Максимум 1k символа. Забележка: Потребителското име не може да бъде имейл. Не трябва да е уникално. **/
    username: string;
    /** По избор. Максимум 3k символа за URL адреси. По подразбиране е от gravatar на базата на имейла. Поддържа 64 кодирани изображения, в този случай лимитът е 50k символа. **/ 
    avatar?: string;
    /** По избор. По подразбиране false. **/
    optedInNotifications?: boolean;
    /** По избор. По подразбиране false. **/
    optedInSubscriptionNotifications?: boolean;
    /** По избор. Максимум 100 символа. Този етикет ще бъде показан до името им. По подразбиране е Administrator/Moderator, когато е приложимо. **/
    displayLabel?: string;
    /** По избор. Максимум 500 символа. Това ще бъде показано вместо потребителското име. **/
    displayName?: string;
    /** По избор. Максимум 2k символа. Името на потребителя ще води към това. **/
    websiteUrl?: string;
    /** По избор. До 100 групи на потребител. Идентификаторът на група не може да е по-дълъг от 50 символа. **/
    groupIds?: string[];
    /** По избор. Означава потребителя като администратор. **/
    isAdmin?: boolean;
    /** По избор. Означава потребителя като модератор. **/
    isModerator?: boolean;
    /** По избор, по подразбиране true. Задайте false, за да активирате таба "activity" в профила на потребителя. **/
    isProfileActivityPrivate?: boolean;
    /** По избор, по подразбиране false. Задайте true, за да деактивирате коментарите в профила. **/
    isProfileCommentsPrivate?: boolean;
    /** По избор, по подразбиране false. Задайте true, за да деактивирате директните съобщения към този потребител. **/
    isProfileDMDisabled?: boolean;
    /** По избор конфигурация за значки на потребителя. **/
    badgeConfig?: {
        /** Масив от глобални ID-та на значки за присвояване. Ограничено до 30 значки. Редът се зачита. **/
        badgeIds: string[];
        /** Масив от ID-та на значки, обхванати от текущата страница (urlId). Показват се само на присвоената страница. **/
        pageBadgeIds?: string[];
        /** Ако е true, заменя съществуващите показвани значки. Глобалните и обхванатите от страница се презаписват независимо. **/
        override?: boolean;
        /** Ако е true, обновява свойствата за показване на значките от конфигурацията на tenant. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Модератори и администратори

За администратори и модератори подайте съответните флагове `isAdmin` или `isModerator` в обекта `SSOUser`.

#### Известия

За да активирате или деактивирате известията, задайте стойността на `optedInNotifications` съответно на `true` или `false`. Първия път, когато потребителят зареди страницата с тази стойност в SSO payload-а, настройките му за известия ще бъдат обновени.

Освен това, ако искате потребителите да получават имейли за известия за активност на страници, които следят (вместо само известия в приложението), задайте `optedInSubscriptionNotifications` на `true`.

#### VIP потребители и специални етикети

Можете да покажете специален етикет до името на потребителя, като използвате опционалното поле "displayLabel".

#### Неудентифицирани потребители

За да представите неудентифициран потребител, просто не попълвайте userDataJSONBase64, verificationHash или timestamp. Предоставете loginURL.

Тези потребители няма да могат да коментират, а вместо това ще им бъде показано съобщение за влизане (съобщение, линк или бутон, в зависимост от конфигурацията).

#### Директни примери за сериализация и хеширане на потребителски данни

Повече подробности като примери <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/node-express/routes/index.js#L23" target="_blank">тук</a> (js), <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/java-springboot/src/main/java/com/winricklabs/ssodemo/DemoController.java#L68" target="_blank">тук</a> (java) и <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">тук</a> (php).

Разбираме, че всяка интеграция може да бъде сложен и болезнен процес. Не се колебайте да се свържете с вашия представител или да използвате <a href="https://fastcomments.com/auth/my-account/help" target="_blank">страницата за поддръжка</a>.