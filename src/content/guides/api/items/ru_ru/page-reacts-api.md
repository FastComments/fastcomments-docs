Page Reacts позволяют вашим пользователям ставить лайк странице или реагировать на неё с помощью собственного набора изображений реакций. Виджет [Page Reacts widget](/guide-page-reacts.html) и виджет Floating Likes построены на этих конечных точках, и вы можете вызывать их самостоятельно, чтобы создать собственную кнопку лайка.

В отличие от остальной части этого руководства, конечные точки Page Reacts являются публичными. Они вызываются из браузеров ваших пользователей, не требуют API‑ключа и не расходуют API‑кредиты. Каждая реакция принадлежит пользователю, сделавшему запрос, поэтому пользователь может добавлять или удалять только свои реакции.

Существует два набора конечных точек:

- `/page-reacts/v1/likes/:tenantId` — один «лайк» на пользователя на страницу. Используйте их для кнопки лайка.
- `/page-reacts/v2/:tenantId` — несколько реакций на страницу, каждая из которых идентифицируется коротким `id`, выбранным вами (например `heart` или `laugh`).

Обе доступны также в наших SDK как часть `PublicApi`, например `getV1PageLikes`, `createV1PageReact` и `deleteV1PageReact` в [JavaScript SDK](/guide-sdk-javascript.html).

### Идентификация пользователя

Реакции привязаны к пользователю, сделавшему запрос:

- **SSO‑пользователи:** передайте параметр запроса `sso`, содержащий URI‑закодированный JSON того же объекта SSO, который вы передаёте виджету комментариев. См. раздел [SSO](/guide-customizations-and-configuration.html#sso).
- **Анонимные пользователи:** когда параметр `sso` отсутствует и нет входа в FastComments, сервер назначает браузеру анонимный идентификатор, хранящийся в cookie‑сеансе FastComments. Отправляйте запросы с `credentials: 'include'`, чтобы cookie сохранялась между запросами. Браузеры, блокирующие сторонние cookie, не сохранят анонимный идентификатор, поэтому используйте SSO, если каждый пользователь должен быть надёжно распознан.

### Идентификатор urlId

`urlId` идентифицирует страницу, так же как и для комментариев. Используйте тот же `urlId`, который вы передаёте виджету комментариев, чтобы лайки и комментарии учитывались на одной и той же странице. Не забудьте URI‑закодировать его.

[inline-code-attrs-start title = 'Пример кнопки лайка'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// Необязательно, для SSO‑пользователей. Тот же объект, который вы передаёте в параметр "sso" виджета комментариев.
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