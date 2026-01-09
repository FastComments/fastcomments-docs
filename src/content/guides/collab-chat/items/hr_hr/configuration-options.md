### Overview

FastComments Collab Chat proširuje standardni FastComments widget za komentare, pa nasljeđuje sve opcije konfiguracije iz osnovnog widgeta dok dodaje nekoliko opcija specifičnih za tekstualne bilješke.

### Required Configuration

#### tenantId

Zahtijeva se vaš FastComments Tenant ID. Možete ga pronaći u vašoj [nadzornoj ploči FastComments](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Primjer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Collab Chat Specific Options

#### urlId

Po zadanim postavkama, Collab Chat generira jedinstveni identifikator za svaki razgovor na temelju URL-a stranice, DOM putanje do elementa i odabranog raspona teksta. Ovo možete zamijeniti prilagođenim `urlId`.

[inline-code-attrs-start title = "Primjer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Ovo je korisno kada se struktura URL-a može promijeniti, a želite zadržati iste razgovore, ili kada želite dijeliti bilješke između više stranica.

#### topBarTarget

Kontrolira prikaz gornje trake koja prikazuje broj korisnika i broj rasprava. Postavite na `null` da u potpunosti onemogućite gornju traku, ili navedite DOM element u koji će se prikazati na određenoj lokaciji.

[inline-code-attrs-start title = "Primjer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Onemogući gornju traku
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Prikaži gornju traku na prilagođenom mjestu
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

Omogućite stilizaciju za tamni način kada vaša stranica ima tamnu pozadinu. Ovo otkrivanje je automatsko, ali ponekad je poželjno to nadjačati.

[inline-code-attrs-start title = "Primjer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Povratna funkcija (callback) koja se poziva kad god se broj komentara promijeni. Ovo je korisno za ažuriranje elemenata sučelja kao što su značke (badges) ili naslovi stranica.

[inline-code-attrs-start title = "Primjer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### Inherited Configuration Options

Budući da Collab Chat proširuje standardni widget za komentare, možete koristiti bilo koju konfiguracijsku opciju iz osnovnog FastComments widgeta. Evo nekoliko često korištenih opcija:

#### locale

Postavite jezik za korisničko sučelje widgeta. FastComments podržava desetke jezika.

[inline-code-attrs-start title = "Primjer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Španjolski
});
[inline-code-end]

#### readonly

Učinite sve razgovore samo za čitanje. Korisnici mogu pregledavati postojeće bilješke, ali ne mogu stvarati nove ni odgovarati.

[inline-code-attrs-start title = "Primjer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso and simpleSSO

Integrirajte se sa vašim sustavom autentifikacije koristeći Single Sign-On.

[inline-code-attrs-start title = "Primjer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // Konfiguracija SSO
    }
});
[inline-code-end]

Pogledajte dokumentaciju za SSO za potpune detalje o opcijama autentifikacije.

#### maxReplyDepth

Kontrolirajte koliko razina ugniježđenih odgovora je dopušteno. Po zadanoj postavci, Collab Chat postavlja ovo na 0, što znači da su svi komentari ravni (bez ugniježđenih odgovora). To možete promijeniti ako želite razgovore u nitima.

[inline-code-attrs-start title = "Primjer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Dopusti 3 razine ugniježđivanja
});
[inline-code-end]

### Internal Configuration

Ove opcije automatski postavlja Collab Chat i ne bi se trebale nadjačavati:

The `productId` is automatically set to `3` for Collab Chat. The `floating-chat` extension is automatically loaded to provide the chat window functionality. The widget automatically detects mobile devices (screens under 768px wide) and adjusts the UI accordingly.

### Complete Example

Evo primjera koji prikazuje više opcija konfiguracije zajedno:

[inline-code-attrs-start title = "Primjer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
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
        // Vaša SSO konfiguracija
    },
    maxReplyDepth: 1
});
[inline-code-end]

Za potpuni popis svih dostupnih opcija konfiguracije naslijeđenih iz osnovnog widgeta, pogledajte glavnu FastComments dokumentaciju o konfiguraciji.