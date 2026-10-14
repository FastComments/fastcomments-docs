Page Reacts омогућава вашим корисницима да лајкују страницу, или реагују на њу помоћу вашег скупа слика за реакције. The [Page Reacts widget](/guide-page-reacts.html) and the Floating Likes widget are built on these endpoints, and you can call them yourself to build your own like button.

За разлику од остатка овог водича, Page Reacts крајње тачке су јавне. Позивају се из прегледача ваших корисника, не захтевају API кључ и не троше API кредите. Свака реакција припада кориснику који је послао захтев, тако да корисник може да дода или уклони само своје реакције.

Постоје два скупа крајњих тачака:

- `/page-reacts/v1/likes/:tenantId` – један „лајк“ по кориснику по страници. Користите ово за дугме за лајк.
- `/page-reacts/v2/:tenantId` – више реакција по страници, свака идентификује се кратким `id` који изаберете (на пример `heart` или `laugh`).

Обе су такође доступне у нашим SDK-овима као део `PublicApi`, на пример `getV1PageLikes`, `createV1PageReact` и `deleteV1PageReact` у [JavaScript SDK](/guide-sdk-javascript.html).

### Showing Who Reacted

Проследите `includeUserInfo=public` у `GET` крајње тачке да бисте такође добили кориснике који су лајковали или реаговали, са њиховим корисничким именима и аватарима, прво најновије. Корисници који су поставили своју профилну активност као приватну се изостављају, као и анонимни корисници. Враћа се до 100 реакција.

SSO корисници се креирају са приватном профилном активностом осим ако ваш SSO payload не постави `isProfileActivityPrivate` на `false`, па то поставите за кориснике које желите да се појаве у овим листама.

На страницама које захтевају SSO, само SSO корисници који могу да виде страницу могу да листају ко је реаговао. Остали добијају `403` са кодом `sso-required` или `page-forbidden`.

Са вашег сервера, [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) и [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) враћају исте листе уз API кључ, и такође прихватају `includeUserInfo=include-private` да укључе кориснике са приватним профилима.

### Identifying the User

Реакције су везане за корисника који шаље захтев:

- **SSO users:** проследите `sso` параметар упита, постављен на URI‑енкодирани JSON истог SSO објекта који дајете widget‑у за коментаре. Погледајте [SSO](/guide-customizations-and-configuration.html#sso).
- **Anonymous users:** када нема `sso` параметра и нема FastComments пријаве, сервер додељује прегледачу анонимни id који се чува у FastComments сесијском колачићу. Шаљите захтеве са `credentials: 'include'` како би колачић остао између захтева. Прегледачи који блокирају колачиће трећих страна неће задржати анонимни id, па користите SSO када сваки корисник мора бити поуздано препознат.

### The urlId

`urlId` идентификује страницу, исто као и за коментаре. Користите исти `urlId` који дајете widget‑у за коментаре како би лајкови и коментари били бројани на истој страници. Не заборавите да га URI‑енкодујете.

[inline-code-attrs-start title = 'Пример дугмета за лајк'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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