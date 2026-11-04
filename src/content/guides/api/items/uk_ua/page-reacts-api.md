Віджети Page Reacts дозволяють вашим користувачам вподобати сторінку або відреагувати на неї за допомогою власного набору зображень реакцій. [Віджет Page Reacts](/guide-page-reacts.html) та віджет Floating Likes побудовані на цих ендпоінтах, і ви можете викликати їх самостійно, щоб створити власну кнопку вподобайок.

На відміну від решти цього посібника, ендпоінти Page Reacts є публічними. Вони викликаються з браузерів ваших користувачів, не потребують API-ключа і не витрачають API-кредити. Кожна реакція належить користувачу, який виконує запит, тому користувач може додавати або видаляти лише власні реакції.

Існує два набори ендпоінтів:

- `/page-reacts/v1/likes/:tenantId` - одна "вподобайка" на користувача на сторінку. Використовуйте їх для кнопки вподобайок.
- `/page-reacts/v2/:tenantId` - кілька реакцій на сторінку, кожна ідентифікується коротким `id`, який ви обираєте (наприклад, `heart` або `laugh`).

Обидва набори також доступні в наших SDK як частина `PublicApi`, наприклад `getV1PageLikes`, `createV1PageReact` та `deleteV1PageReact` у [JavaScript SDK](/guide-sdk-javascript.html).

### Показ тих, хто відреагував

Передайте `includeUserInfo=public` до ендпоінтів `GET`, щоб також отримати користувачів, які вподобали або відреагували, з їхніми іменами користувачів та аватарами, починаючи з найновіших. Користувачі, які зробили свою активність профілю приватною, виключаються, як і анонімні користувачі. Повертається до 100 реакцій.

SSO-користувачі створюються з приватною активністю профілю, якщо ваш SSO-payload не встановлює `isProfileActivityPrivate` у `false`, тому встановіть це для користувачів, яких ви хочете бачити в цих списках.

На сторінках, що вимагають SSO, лише SSO-користувачі, які можуть переглядати сторінку, можуть бачити список тих, хто відреагував. Інші отримують `403` з кодом `sso-required` або `page-forbidden`.

З вашого сервера [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) та [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) повертають ті самі списки з API-ключем, а також приймають `includeUserInfo=include-private`, щоб включити користувачів з приватними профілями.

### Ідентифікація користувача

Реакції прив'язані до користувача, який виконує запит:

- **SSO-користувачі:** передайте параметр запиту `sso`, встановлений у URI-кодований JSON того самого SSO-об'єкта, який ви надаєте віджету коментарів. Дивіться [SSO](/guide-customizations-and-configuration.html#sso).
- **Анонімні користувачі:** коли немає параметра `sso` та немає входу в FastComments, сервер призначає браузеру анонімний id, що зберігається в cookie сесії FastComments. Надсилайте запити з `credentials: 'include'`, щоб cookie зберігався між запитами. Браузери, які блокують сторонні cookie, не зберігатимуть анонімний id, тому використовуйте SSO, коли кожного користувача потрібно надійно розпізнавати.

### urlId

`urlId` ідентифікує сторінку так само, як і для коментарів. Використовуйте той самий `urlId`, який ви надаєте віджету коментарів, щоб вподобайки та коментарі враховувалися на одній сторінці. Не забудьте URI-кодувати його.

[inline-code-attrs-start title = 'Приклад кнопки вподобайок'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// Необов'язково, для SSO-користувачів. Той самий об'єкт, який ви надаєте опції "sso" віджета коментарів.
const sso = null;

function likesUrl() {
    let url = `https://fastcomments.com/page-reacts/v1/likes/${tenantId}?urlId=${encodeURIComponent(urlId)}`;
    if (sso) {
        url += '&sso=' + encodeURIComponent(JSON.stringify(sso));
    }
    return url;
}

async function getLikes() {
    const response = await fetch(likesUrl(), {credentials: 'include'});
    return response.json(); // {status, likeCount, didLike, commentCount, urlIdWS}
}

async function like() {
    await fetch(likesUrl(), {method: 'POST', credentials: 'include'});
}

async function unlike() {
    await fetch(likesUrl(), {method: 'DELETE', credentials: 'include'});
}
[inline-code-end]