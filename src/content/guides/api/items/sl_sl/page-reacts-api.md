Page Reacts omogočajo vašim uporabnikom, da všečajo stran ali se na njo odzovejo s svojimi slikami reakcij. The [Page Reacts widget](/guide-page-reacts.html) and the Floating Likes widget are built on these endpoints, and you can call them yourself to build your own like button.

Za razliko od preostalega vodnika so končne točke Page Reacts javne. Kličete jih iz brskalnikov vaših uporabnikov, ne zahtevajo API ključa in ne porabijo API kreditov. Vsaka reakcija pripada uporabniku, ki je poslal zahtevo, zato lahko uporabnik doda ali odstrani le svoje reakcije.

Obstajata dva niza končnih točk:

- `/page-reacts/v1/likes/:tenantId` - en sam "like" na uporabnika na stran. Uporabite to za gumb všečkov.
- `/page-reacts/v2/:tenantId` - več reakcij na stran, vsaka identificirana s kratkim `id`, ki ga izberete (na primer `heart` ali `laugh`).

Obe sta na voljo tudi v naših SDK-jih kot del `PublicApi`, na primer `getV1PageLikes`, `createV1PageReact` in `deleteV1PageReact` v [JavaScript SDK](/guide-sdk-javascript.html).

### Prikazovanje, kdo se je odzval

Posredujte `includeUserInfo=public` na `GET` končne točke, da poleg tega dobite tudi uporabnike, ki so všečkali ali se odzvali, z njihovimi uporabniškimi imeni in avatarji, najprej najnovejše. Uporabniki, ki so svojo aktivnost profila nastavili kot zasebno, so izključeni, prav tako anonimni uporabniki. Vrne se do 100 reakcij.

Uporabniki SSO so ustvarjeni z zasebno aktivnostjo profila, razen če vaš SSO payload nastavi `isProfileActivityPrivate` na `false`, zato to nastavite za uporabnike, ki naj se pojavijo na teh seznamih.

Na straneh, ki zahtevajo SSO, lahko le SSO uporabniki, ki lahko vidijo stran, dobijo seznam, kdo se je odzval. Drugi prejmejo `403` s kodo `sso-required` ali `page-forbidden`.

Z vašega strežnika, [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) in [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) vrneta enake sezname z API ključem ter sprejmeta tudi `includeUserInfo=include-private`, da vključita uporabnike z zasebnimi profili.

### Identifikacija uporabnika

Reakcije so vezane na uporabnika, ki pošilja zahtevo:

- **SSO uporabniki:** posredujte poizvedbeni parameter `sso`, nastavljen na URI kodiran JSON istega SSO objekta, ki ga podate widgetu za komentarje. Glejte [SSO](/guide-customizations-and-configuration.html#sso).
- **Anonimni uporabniki:** ko ni parametra `sso` in ni FastComments prijave, strežnik brskalniku dodeli anonimni ID, shranjen v FastComments sejnskem piškotku. Pošiljajte zahteve z `credentials: 'include'`, da se piškotek ohrani med zahtevami. Brskalniki, ki blokirajo piškotke tretjih oseb, ne bodo ohranili anonimnega ID-ja, zato uporabite SSO, ko mora biti vsak uporabnik zanesljivo prepoznan.

### urlId

`urlId` identificira stran, enako kot pri komentarjih. Uporabite isti `urlId`, ki ga podate widgetu za komentarje, da se všečki in komentarji štejejo na isti strani. Ne pozabite ga URI kodirati.

[inline-code-attrs-start title = 'Primer gumba všečkov'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// Opcijsko, za SSO uporabnike. Enak objekt, ki ga podate widgetu za komentarje v možnosti "sso".
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