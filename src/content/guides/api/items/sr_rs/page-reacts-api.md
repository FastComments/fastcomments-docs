Page Reacts омогућавају вашим корисницима да лајкују страницу или да реагују на њу са вашим сопственим скупом слика реакција. [Page Reacts виџет](/guide-page-reacts.html) и Floating Likes виџет су изграђени на овим endpoint-има, а можете их сами позивати да направите своје дугме за лајковање.

За разлику од остатка овог водича, Page Reacts endpoint-и су јавни. Они се позивају из прегледача ваших корисника, не захтевају API кључ и не коштају API кредите. Свака реакција припада кориснику који шаље захтев, тако да корисник може само да дода или уклони своју.

Постоје два скупа endpoint-а:

- `/page-reacts/v1/likes/:tenantId` - један "лајк" по кориснику по страници. Користите ово за дугме за лајковање.
- `/page-reacts/v2/:tenantId` - више реакција по страници, од којих је свака идентификована кратким `id`-јем који ви бирате (на пример `heart` или `laugh`).

Оба су такође доступна у нашим SDK-овима као део `PublicApi`, на пример `getV1PageLikes`, `createV1PageReact` и `deleteV1PageReact` у [JavaScript SDK](/guide-sdk-javascript.html).

### Приказивање ко је реаговао

Проследите `includeUserInfo=public` на `GET` endpoint-е да бисте такође добили кориснике који су лајковали или реаговали, са њиховим корисничким именима и аватарима, најскорији прво. Корисници који су учинили активност свог профила приватном су изостављени, као и анонимни корисници. Враћа се до 100 реакција.

SSO корисници се креирају са приватном активношћу профила осим ако ваш SSO payload не постави `isProfileActivityPrivate` на `false`, па то поставите за кориснике које желите да се појављују у овим листама.

На страницама које захтевају SSO, само SSO корисници који могу да виде страницу могу да наведу ко је реаговао. Остали добијају `403` са кодом `sso-required` или `page-forbidden`.

Са вашег сервера, [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) и [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) враћају исте листе са API кључем, а такође прихватају `includeUserInfo=include-private` да укључе кориснике са приватним профилима.

### Идентификовање корисника

Реакције су везане за корисника који шаље захтев:

- **SSO корисници:** проследите `sso` query параметар, постављен на URI кодирани JSON истог SSO објекта који дајете коментар виџету. Видите [SSO](/guide-customizations-and-configuration.html#sso).
- **Анонимни корисници:** када нема `sso` параметра ни FastComments пријаве, сервер додељује прегледачу анонимни id ускладиштен у FastComments session колачићу. Шаљите захтеве са `credentials: 'include'` да би колачић био задржан између захтева. Прегледачи који блокирају third-party колачиће неће задржати анонимни id, па користите SSO када сваки корисник мора поуздано бити препознат.

### urlId

`urlId` идентификује страницу, исто као и за коментаре. Користите исти `urlId` који дајете коментар виџету да би лајкови и коментари били бројани на истој страници. Не заборавите да га URI кодирате.

[inline-code-attrs-start title = 'Пример дугмета за лајковање'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// Опционо, за SSO кориснике. Исти објекат који дајете "sso" опцији коментар виџета.
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