Page Reacts омогућава вашим корисницима да лајкују страницу или реагују на њу помоћу вашег скупа слика за реакције. Видгет [Page Reacts widget](/guide-page-reacts.html) и видгет Floating Likes засновани су на овим крајњим тачкама, а ви их можете позивати сами да направите сопствени дугме за лајк.

За разлику од осталог овог водича, крајње тачке Page Reacts су јавне. Позивају се из прегледача ваших корисника, не захтевају API кључ и не троше API кредите. Свака реакција припада кориснику који је послао захтев, тако да корисник може само да дода или уклони своје реакције.

Постоје два скупа крајњих тачака:

- `/page-reacts/v1/likes/:tenantId` – један „лајк“ по кориснику по страници. Користите ово за дугме за лајк.
- `/page-reacts/v2/:tenantId` – више реакција по страници, свака идентификује се кратким `id` који изаберете (на пример `heart` или `laugh`).

Обе су такође доступне у нашим SDK-има као део `PublicApi`, на пример `getV1PageLikes`, `createV1PageReact` и `deleteV1PageReact` у [JavaScript SDK](/guide-sdk-javascript.html).

### Идентификовање корисника

Реакције су везане за корисника који шаље захтев:

- **SSO корисници:** проследите параметар упита `sso`, постављен на URI‑енкодирани JSON истог SSO објекта који дајете видгету за коментаре. Погледајте [SSO](/guide-customizations-and-configuration.html#sso).
- **Анонимни корисници:** када нема параметра `sso` и нема FastComments пријаве, сервер додељује прегледачу анонимни ID који се чува у FastComments сесијском колачићу. Шаљите захтеве са `credentials: 'include'` како би колачић био задржан између захтева. Прегледачи који блокирају колачиће трећих страна неће задржати анонимни ID, па користите SSO када је потребно поуздано препознавање сваког корисника.

### urlId

`urlId` идентификује страницу, исто као и за коментаре. Користите исти `urlId` који дајете видгету за коментаре како би лајкови и коментари били бројани на истој страници. Не заборавите да га URI‑енкодујете.

[inline-code-attrs-start title = 'Пример дугмета за лајк'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// Опционо, за SSO кориснике. Исти објекат који дајете опцији "sso" видгета за коментаре.
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