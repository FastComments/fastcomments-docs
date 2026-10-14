Page Reacts позволяют вашим пользователям ставить лайк странице или реагировать на неё с помощью собственного набора изображений реакций. Виджет [Page Reacts widget](/guide-page-reacts.html) и виджет Floating Likes построены на этих конечных точках, и вы можете вызывать их самостоятельно, чтобы создать собственную кнопку лайка.

В отличие от остальной части руководства, конечные точки Page Reacts являются публичными. Они вызываются из браузеров ваших пользователей, не требуют API‑ключа и не тратят API‑кредиты. Каждая реакция принадлежит пользователю, сделавшему запрос, поэтому пользователь может добавлять или удалять только свои реакции.

Существует два набора конечных точек:

- `/page-reacts/v1/likes/:tenantId` — один «лайк» на пользователя на страницу. Используйте их для кнопки лайка.
- `/page-reacts/v2/:tenantId` — несколько реакций на страницу, каждая из которых идентифицируется коротким `id`, выбранным вами (например `heart` или `laugh`).

Обе доступны также в наших SDK в составе `PublicApi`, например `getV1PageLikes`, `createV1PageReact` и `deleteV1PageReact` в [JavaScript SDK](/guide-sdk-javascript.html).

### Показать, кто отреагировал

Передайте `includeUserInfo=public` в `GET`‑запросы, чтобы также получить пользователей, которые поставили лайк или отреагировали, с их именами пользователей и аватарами, в порядке от самых новых. Пользователи, сделавшие свою активность профиля приватной, а также анонимные пользователи, исключаются. Возвращается до 100 реакций.

Пользователи SSO создаются с приватной активностью профиля, если только ваш SSO‑payload не устанавливает `isProfileActivityPrivate` в `false`, поэтому установите его для пользователей, которых хотите видеть в этих списках.

На страницах, требующих SSO, только пользователи SSO, имеющие доступ к странице, могут получить список тех, кто отреагировал. Остальные получают `403` с кодом `sso-required` или `page-forbidden`.

С вашего сервера, [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) и [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) возвращают те же списки с использованием API‑ключа, а также принимают `includeUserInfo=include-private`, чтобы включать пользователей с приватными профилями.

### Идентификация пользователя

Реакции привязаны к пользователю, сделавшему запрос:

- **Пользователи SSO:** передайте параметр запроса `sso`, содержащий URI‑закодированный JSON того же SSO‑объекта, который вы передаёте виджету комментариев. См. [SSO](/guide-customizations-and-configuration.html#sso).
- **Анонимные пользователи:** когда параметр `sso` отсутствует и нет входа в FastComments, сервер назначает браузеру анонимный идентификатор, хранящийся в cookie сессии FastComments. Отправляйте запросы с `credentials: 'include'`, чтобы cookie сохранялась между запросами. Браузеры, блокирующие сторонние cookie, не сохранят анонимный идентификатор, поэтому используйте SSO, когда каждый пользователь должен быть надёжно распознан.

### Идентификатор urlId

`urlId` идентифицирует страницу, так же как и для комментариев. Используйте тот же `urlId`, который вы передаёте виджету комментариев, чтобы лайки и комментарии учитывались на одной странице. Не забудьте закодировать его в URI.

[inline-code-attrs-start title = 'Пример кнопки лайка'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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