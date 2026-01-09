### Pregled

FastComments Collab Chat razširja standardni pripomoček za komentarje FastComments, zato deduje vse možnosti konfiguracije iz osnovnega gradnika in dodaja nekaj, specifičnih za besedilne oznake.

### Zahtevana konfiguracija

#### tenantId

Za uporabo potrebujete vaš FastComments Tenant ID. Najdete ga v vaši [FastComments nadzorni plošči](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Možnosti, specifične za Collab Chat

#### urlId

Privzeto Collab Chat ustvari edinstven identifikator za vsako pogovor glede na URL strani, DOM pot do elementa in izbrani obseg besedila. To lahko prevedete z lastnim `urlId`.

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

To je uporabno, kadar se lahko struktura vaših URL-jev spremeni, vendar želite ohraniti iste pogovore, ali kadar želite deliti oznake med več stranmi.

#### topBarTarget

Nadzoruje prikaz zgornje vrstice, ki prikazuje število uporabnikov in število razprav. Nastavite na `null`, da popolnoma onemogočite zgornjo vrstico, ali navedite DOM element, da jo prikažete na določenem mestu.

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Onemogoči zgornjo vrstico
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Prikaži zgornjo vrstico na prilagojenem mestu
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

Omogočite temno oblikovanje, ko ima vaša stran temno ozadje. To zaznavanje je samodejno, vendar ga včasih želite preseči.

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Funkcija povratnega klica, ki se sproži vsakič, ko se število komentarjev spremeni. To je uporabno za posodabljanje elementov uporabniškega vmesnika, kot so značke ali naslovi strani.

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### Podedovane možnosti konfiguracije

Ker Collab Chat razširja standardni pripomoček za komentarje, lahko uporabite katero koli konfiguracijsko možnost iz osnovnega FastComments gradnika. Tukaj je nekaj pogosto uporabljenih možnosti:

#### locale

Nastavite jezik za uporabniški vmesnik gradnika. FastComments podpira desetine jezikov.

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // španščina
});
[inline-code-end]

#### readonly

Naredite vse pogovore samo za branje. Uporabniki lahko ogledajo obstoječe oznake, vendar ne morejo ustvarjati novih ali odgovarjati.

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso and simpleSSO

Integrirajte z vašim sistemom za overjanje z uporabo Single Sign-On.

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // Konfiguracija SSO
    }
});
[inline-code-end]

Oglejte si dokumentacijo za SSO za popolne podrobnosti o možnostih overjanja.

#### maxReplyDepth

Nadzoruje, kako globoko se lahko odgovori gnezdo. Privzeto Collab Chat nastavi to na 0, kar pomeni, da so vsi komentarji ploski (brez gnezdenih odgovorov). To lahko spremenite, če želite navite pogovore.

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Dovoli 3 nivoje gnezdenja
});
[inline-code-end]

### Notranja konfiguracija

Te možnosti nastavi Collab Chat samodejno in jih ne smete preglasiti:

The `productId` is automatically set to `3` for Collab Chat. The `floating-chat` extension is automatically loaded to provide the chat window functionality. The widget automatically detects mobile devices (screens under 768px wide) and adjusts the UI accordingly.

### Celoten primer

Tukaj je primer, ki prikazuje več konfiguracijskih možnosti skupaj:

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(document.getElementById('article'), {
    tenantId: 'demo',
    urlId: 'my-article-v2',
    hasDarkBackground: false,
    locale: 'en',
    topBarTarget: document.getElementById('custom-header'),
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) My Article` : 'My Article';
    },
    sso: {
        // Vaša konfiguracija SSO
    },
    maxReplyDepth: 1
});
[inline-code-end]

Za popoln seznam vseh razpoložljivih možnosti konfiguracije, podedovanih iz osnovnega gradnika, glejte glavno FastComments dokumentacijo o konfiguraciji.

---