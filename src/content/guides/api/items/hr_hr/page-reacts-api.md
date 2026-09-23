Page Reacts omogućuje vašim korisnicima da lajkuju stranicu ili reagiraju na nju s vlastitim skupom slika reakcija. Widget [Page Reacts](/guide-page-reacts.html) i widget Floating Likes izgrađeni su na tim krajnjim točkama, a vi ih možete pozvati sami kako biste izradili vlastiti gumb za lajk.

Za razliku od ostatka ovog vodiča, krajnje točke Page Reacts su javne. Pozivaju se iz preglednika vaših korisnika, ne zahtijevaju API ključ i ne troše API kredite. Svaka reakcija pripada korisniku koji je poslao zahtjev, pa korisnik može dodati ili ukloniti samo svoje reakcije.

Postoje dva skupa krajnjih točaka:

- `/page-reacts/v1/likes/:tenantId` – jedan “like” po korisniku po stranici. Koristite ih za gumb za lajk.
- `/page-reacts/v2/:tenantId` – više reakcija po stranici, svaka identificirana kratkim `id`‑om koji odaberete (na primjer `heart` ili `laugh`).

Obje su također dostupne u našim SDK‑ovima kao dio `PublicApi`, na primjer `getV1PageLikes`, `createV1PageReact` i `deleteV1PageReact` u [JavaScript SDK](/guide-sdk-javascript.html).

### Identifikacija korisnika

Reakcije su vezane uz korisnika koji šalje zahtjev:

- **SSO korisnici:** proslijedite `sso` parametar upita, postavljen na URI‑kodirani JSON istog SSO objekta koji dajete widgetu za komentare. Pogledajte [SSO](/guide-customizations-and-configuration.html#sso).
- **Anonimni korisnici:** kada ne postoji `sso` parametar i nema FastComments prijave, poslužitelj dodjeljuje pregledniku anonimni ID pohranjen u FastComments sesijskom kolačiću. Šaljite zahtjeve s `credentials: 'include'` kako bi se kolačić zadržao između zahtjeva. Preglednici koji blokiraju kolačiće trećih strana neće zadržati anonimni ID, stoga koristite SSO kada svaki korisnik mora biti pouzdano prepoznat.

### urlId

`urlId` identificira stranicu, isto kao i za komentare. Koristite isti `urlId` koji dajete widgetu za komentare kako bi se lajkovi i komentari brojili na istoj stranici. Zapamtite da ga URI‑kodirate.

[inline-code-attrs-start title = 'Primjer gumba za lajk'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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