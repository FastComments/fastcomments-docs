Page Reacts дозволяє вашим користувачам лайкати сторінку або реагувати на неї за допомогою власного набору зображень реакцій. Віджет [Page Reacts widget](/guide-page-reacts.html) та віджет Floating Likes побудовані на цих кінцевих точках, і ви можете викликати їх самостійно, щоб створити власну кнопку лайка.

На відміну від решти цього посібника, кінцеві точки Page Reacts є публічними. Вони викликаються з браузерів ваших користувачів, не потребують API‑ключа і не вимагають API‑кредитів. Кожна реакція належить користувачу, який робить запит, тому користувач може додавати або видаляти лише свої реакції.

Існує два набори кінцевих точок:

- `/page-reacts/v1/likes/:tenantId` — один «лайк» на користувача на сторінку. Використовуйте їх для кнопки лайка.
- `/page-reacts/v2/:tenantId` — кілька реакцій на сторінку, кожна ідентифікується коротким `id`, який ви обираєте (наприклад `heart` або `laugh`).

Обидва також доступні в наших SDK як частина `PublicApi`, наприклад `getV1PageLikes`, `createV1PageReact` та `deleteV1PageReact` у [JavaScript SDK](/guide-sdk-javascript.html).

### Визначення користувача

Реакції прив'язані до користувача, який робить запит:

- **SSO користувачі:** передайте параметр запиту `sso`, встановлений у URI‑закодований JSON того ж SSO‑об’єкта, який ви передаєте віджету коментарів. Дивіться [SSO](/guide-customizations-and-configuration.html#sso).
- **Анонімні користувачі:** коли немає параметра `sso` і немає входу в FastComments, сервер призначає браузеру анонімний ідентифікатор, збережений у cookie сесії FastComments. Надсилайте запити з `credentials: 'include'`, щоб cookie зберігався між запитами. Браузери, які блокують сторонні cookie, не збережуть анонімний ідентифікатор, тому використовуйте SSO, коли кожного користувача потрібно надійно розпізнавати.

### Ідентифікатор urlId

`urlId` ідентифікує сторінку, так само, як і для коментарів. Використовуйте той самий `urlId`, який ви передаєте віджету коментарів, щоб лайки та коментарі підраховувалися на одній і тій же сторінці. Не забудьте закодувати його у URI.

[inline-code-attrs-start title = 'Приклад кнопки лайка'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// Optional, for SSO users. The same object you give the comment widget's "sso" option.
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

---