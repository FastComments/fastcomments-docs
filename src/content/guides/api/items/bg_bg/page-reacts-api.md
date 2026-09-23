Page Reacts позволяват на вашите потребители да харесат страница или да реагират на нея със собствен набор от изображения за реакция. [Page Reacts widget](/guide-page-reacts.html) и уиджетът Floating Likes са построени върху тези крайни точки и можете да ги извикате сами, за да създадете собствен бутон за харесване.

За разлика от останалата част от това ръководство, крайните точки за Page Reacts са публични. Те се извикват от браузърите на вашите потребители, не изискват API ключ и не консумират API кредити. Всяка реакция принадлежи на потребителя, който прави заявката, така че потребителят може да добавя или премахва само собствените си реакции.

Има два набора от крайни точки:

- `/page-reacts/v1/likes/:tenantId` – едно единствено „харесване“ за потребител на страница. Използвайте ги за бутон за харесване.
- `/page-reacts/v2/:tenantId` – множество реакции за страница, всяка идентифицирана с кратко `id`, което избирате (например `heart` или `laugh`).

И двете са също достъпни в нашите SDK‑та като част от `PublicApi`, например `getV1PageLikes`, `createV1PageReact` и `deleteV1PageReact` в [JavaScript SDK](/guide-sdk-javascript.html).

### Идентифициране на потребителя

Реакциите са свързани с потребителя, който прави заявката:

- **SSO потребители:** предайте параметъра за заявка `sso`, зададен като URI‑кодиран JSON на същия SSO обект, който подавате на уиджета за коментари. Вижте [SSO](/guide-customizations-and-configuration.html#sso).
- **Анонимни потребители:** когато няма параметър `sso` и няма вход в FastComments, сървърът присвоява на браузъра анонимен идентификатор, съхраняван в сесийния бисквит на FastComments. Изпращайте заявки с `credentials: 'include'`, за да се запази бисквитата между заявките. Браузъри, които блокират бисквитки от трети страни, няма да запазят анонимния идентификатор, затова използвайте SSO, когато всеки потребител трябва да бъде надеждно разпознат.

### urlId

`urlId` идентифицира страницата, както и за коментарите. Използвайте същия `urlId`, който подавате на уиджета за коментари, за да се броят харесванията и коментарите на една и съща страница. Не забравяйте да го кодирате като URI.

[inline-code-attrs-start title = 'Пример за бутон Харесване'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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