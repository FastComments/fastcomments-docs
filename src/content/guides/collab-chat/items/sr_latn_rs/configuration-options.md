### Overview

FastComments Collab Chat proširuje standardni FastComments komentarski widget, tako da nasleđuje sve opcije konfigurisanja iz osnovnog widgeta uz dodatak nekoliko opcija specifičnih za tekstualne anotacije.

### Required Configuration

#### tenantId

Vaš FastComments Tenant ID je obavezan. Možete ga pronaći na vašoj [FastComments kontrolnoj tabli](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Collab Chat Specific Options

#### urlId

Po defaultu, Collab Chat generiše jedinstveni identifikator za svaku konverzaciju zasnovan na URL-u stranice, DOM putanji do elementa i izabranom rasponu teksta. Možete ga prebrisati prilagođenim `urlId`.

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Ovo je korisno kada se struktura vaših URL-ova može promeniti, ali želite sačuvati iste konverzacije, ili kada želite deliti anotacije preko više stranica.

#### topBarTarget

Kontroliše prikaz gornje trake koja prikazuje broj korisnika i broj diskusija. Postavite na `null` da potpuno onemogućite gornju traku, ili navedite DOM element da biste je renderovali na određenoj lokaciji.

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Onemogući gornju traku
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Prikaži gornju traku na prilagođenoj lokaciji
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

Omogućite tamni stil kada vaša stranica ima tamnu pozadinu. Ova detekcija je automatska, ali može biti poželjno da je prepišete.

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Funkcija povratnog poziva koja se poziva kad god se promeni broj komentara. Ovo je korisno za ažuriranje UI elemenata kao što su bedževi ili naslovi stranica.

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

### Inherited Configuration Options

Pošto Collab Chat proširuje standardni komentarski widget, možete koristiti bilo koju opciju konfigurisanja iz osnovnog FastComments widgeta. Evo nekoliko često korišćenih opcija:

#### locale

Podesite jezik za UI widgeta. FastComments podržava desetine jezika.

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // španski
});
[inline-code-end]

#### readonly

Učinite sve konverzacije samo za čitanje. Korisnici mogu pregledati postojeće anotacije, ali ne mogu kreirati nove niti odgovarati.

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso and simpleSSO

Integracija sa vašim sistemom autentifikacije korišćenjem Single Sign-On.

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // SSO konfiguracija
    }
});
[inline-code-end]

Pogledajte SSO dokumentaciju za potpune detalje o opcijama autentifikacije.

#### maxReplyDepth

Kontrolišite koliko nivoa duboko mogu ići odgovori. Po defaultu, Collab Chat postavlja ovo na 0, što znači da su svi komentari pljosnati (nema ugnježdenih odgovora). Možete ovo promeniti ako želite threaded konverzacije.

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Dozvoli 3 nivoa ugnježđivanja
});
[inline-code-end]

### Internal Configuration

Ove opcije se automatski postavljaju od strane Collab Chat-a i ne bi trebalo da ih prepisujete:

The `productId` is automatically set to `3` for Collab Chat. The `floating-chat` extension is automatically loaded to provide the chat window functionality. The widget automatically detects mobile devices (screens under 768px wide) and adjusts the UI accordingly.

### Complete Example

Evo primera koji prikazuje više opcija konfigurisanja zajedno:

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
        // Vaša SSO konfiguracija
    },
    maxReplyDepth: 1
});
[inline-code-end]

Za potpuni spisak svih dostupnih opcija konfigurisanja nasledjenih iz osnovnog widgeta, pogledajte glavnu FastComments dokumentaciju o konfiguraciji.