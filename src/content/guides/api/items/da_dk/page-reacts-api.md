Page Reacts lader dine brugere synes godt om en side, eller reagere på den med dit eget sæt af reaktionsbilleder. Den [Page Reacts widget](/guide-page-reacts.html) og Floating Likes‑widgeten er bygget på disse endpoints, og du kan kalde dem selv for at bygge din egen like‑knap.

I modsætning til resten af denne guide er Page Reacts‑endpoints offentlige. De kaldes fra dine brugeres browsere, kræver ingen API‑nøgle og koster ingen API‑kreditter. Hver reaktion tilhører den bruger, der foretager anmodningen, så en bruger kun kan tilføje eller fjerne sine egne.

Der er to sæt af endpoints:

- `/page-reacts/v1/likes/:tenantId` – en enkelt "like" pr. bruger pr. side. Brug disse til en like‑knap.
- `/page-reacts/v2/:tenantId` – flere reaktioner pr. side, hver identificeret ved et kort `id`, du vælger (for eksempel `heart` eller `laugh`).

Begge er også tilgængelige i vores SDK'er som en del af `PublicApi`, for eksempel `getV1PageLikes`, `createV1PageReact` og `deleteV1PageReact` i den [JavaScript SDK](/guide-sdk-javascript.html).

### Identificering af brugeren

Reaktioner er knyttet til den bruger, der foretager anmodningen:

- **SSO‑brugere:** send `sso`‑query‑parameteren, sat til den URI‑kodede JSON af det samme SSO‑objekt, du giver til kommentarfunktionen. Se [SSO](/guide-customizations-and-configuration.html#sso).
- **Anonyme brugere:** når der ikke er nogen `sso`‑parameter og ingen FastComments‑login, tildeler serveren browseren et anonymt id gemt i FastComments‑sessions‑cookien. Send anmodninger med `credentials: 'include'` så cookien bevares mellem anmodninger. Browsere, der blokerer tredjeparts‑cookies, vil ikke bevare det anonyme id, så brug SSO når hver bruger skal genkendes pålideligt.

### urlId‑et

`urlId` identificerer siden, på samme måde som for kommentarer. Brug den samme `urlId`, du giver til kommentarfunktionen, så likes og kommentarer tælles på den samme side. Husk at URI‑kode den.

[inline-code-attrs-start title = 'Eksempel på Like-knap'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// Valgfri, for SSO‑brugere. Det samme objekt, du giver til kommentarfunktionens "sso"-option.
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