### Pregled

FastComments Collab Chat proširuje standardni FastComments widget za komentare, pa nasljeđuje sve konfiguracijske opcije osnovnog widgeta dok istovremeno dodaje nekoliko opcija specifičnih za tekstualne napomene.

### Obavezna konfiguracija

#### tenantId

Vaš FastComments Tenant ID je obavezan. Možete ga pronaći u svom [FastComments nadzornoj ploči](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Primjer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Opcije specifične za Collab Chat

#### urlId

Prema zadanim postavkama, Collab Chat generira jedinstveni identifikator za svaki razgovor na temelju URL-a stranice, DOM putanje do elementa i odabranog raspona teksta. To možete nadjačati prilagođenim `urlId`.

[inline-code-attrs-start title = "Primjer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Ovo je korisno kada se struktura vašeg URL-a može promijeniti, ali želite zadržati iste razgovore, ili kada želite dijeliti napomene na više stranica.

#### topBarTarget

Kontrolira prikaz gornje trake koja prikazuje broj korisnika i broj rasprava. Postavite na `null` da potpuno onemogućite gornju traku, ili navedite DOM element da je prikažete na određenoj lokaciji.

[inline-code-attrs-start title = "Primjer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Disable top bar
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Render top bar in custom location
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

Omogućite stil tamnog načina rada kada vaša stranica ima tamnu pozadinu. Ova detekcija je automatska, ali može biti poželjno da je nadjačate.

[inline-code-attrs-start title = "Primjer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Povratna funkcija koja se aktivira svaki put kada se broj komentara promijeni. Ovo je korisno za ažuriranje UI elemenata poput znački ili naslova stranica.

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

### Naslijeđene konfiguracijske opcije

Budući da Collab Chat proširuje standardni widget za komentare, možete koristiti bilo koju konfiguracijsku opciju iz osnovnog FastComments widgeta. Evo nekoliko često korištenih opcija:

#### locale

Postavite jezik za UI widgeta. FastComments podržava desetke jezika.

[inline-code-attrs-start title = "Primjer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Spanish
});
[inline-code-end]

#### readonly

Učinite sve razgovore samo za čitanje. Korisnici mogu pregledavati postojeće napomene, ali ne mogu stvarati nove niti odgovarati.

[inline-code-attrs-start title = "Primjer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso i simpleSSO

Integrirajte se sa svojim sustavom autentifikacije pomoću jedinstvene prijave (Single Sign-On).

[inline-code-attrs-start title = "Primjer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // SSO configuration
    }
});
[inline-code-end]

Pogledajte SSO dokumentaciju za potpune detalje o opcijama autentifikacije.

#### maxReplyDepth

Kontrolirajte koliko razina dubine odgovori mogu ići. Prema zadanim postavkama, Collab Chat postavlja ovo na 0, što znači da su svi komentari ravni (bez ugniježđenih odgovora). To možete promijeniti ako želite razgovore s nitima.

[inline-code-attrs-start title = "Primjer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Allow 3 levels of nesting
});
[inline-code-end]

### Interna konfiguracija

Ove opcije automatski postavlja Collab Chat i ne bi ih trebalo nadjačavati:

`productId` je automatski postavljen na `3` za Collab Chat. Proširenje `floating-chat` se automatski učitava kako bi omogućilo funkcionalnost prozora za razgovor. Widget automatski detektira mobilne uređaje (zasloni širine ispod 768px) i prilagođava UI sukladno tome.

### Potpuni primjer

Evo primjera koji prikazuje više konfiguracijskih opcija zajedno:

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
        // Your SSO config
    },
    maxReplyDepth: 1
});
[inline-code-end]

Za potpuni popis svih dostupnih konfiguracijskih opcija naslijeđenih iz osnovnog widgeta, pogledajte glavnu FastComments dokumentaciju konfiguracije.