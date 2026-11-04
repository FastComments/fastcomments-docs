### Pregled

FastComments Collab Chat proširuje standardni FastComments vidžet za komentare, tako da nasleđuje sve konfiguracione opcije osnovnog vidžeta, uz dodavanje nekoliko specifičnih za tekstualne anotacije.

### Obavezna konfiguracija

#### tenantId

Vaš FastComments Tenant ID je obavezan. Možete ga pronaći na svom [FastComments dashboard-u](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Opcije specifične za Collab Chat

#### urlId

Podrazumevano, Collab Chat generiše jedinstveni identifikator za svaku konverzaciju na osnovu URL-a stranice, DOM putanje do elementa i izabranog opsega teksta. Ovo možete zameniti prilagođenim `urlId`.

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Ovo je korisno kada se struktura vašeg URL-a može promeniti, ali želite da zadržite iste konverzacije, ili kada želite da delite anotacije na više stranica.

#### topBarTarget

Kontroliše prikaz gornje trake koja prikazuje broj korisnika i broj diskusija. Postavite na `null` da biste potpuno onemogućili gornju traku, ili navedite DOM element da biste je prikazali na određenoj lokaciji.

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
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

Omogućite stil tamnog režima kada vaša stranica ima tamnu pozadinu. Ova detekcija je automatska, ali može biti poželjno da je zamenite.

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Callback funkcija koja se aktivira svaki put kada se broj komentara promeni. Ovo je korisno za ažuriranje UI elemenata poput bedževa ili naslova stranica.

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

### Nasleđene konfiguracione opcije

Pošto Collab Chat proširuje standardni vidžet za komentare, možete koristiti bilo koju konfiguracionu opciju osnovnog FastComments vidžeta. Evo nekih često korišćenih opcija:

#### locale

Postavite jezik za UI vidžeta. FastComments podržava desetine jezika.

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Spanish
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

#### sso i simpleSSO

Integrišite se sa svojim sistemom autentifikacije korišćenjem Single Sign-On-a.

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
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

Kontrolišite koliko nivoa dubine odgovori mogu ići. Podrazumevano, Collab Chat postavlja ovo na 0, što znači da su svi komentari ravni (bez ugnježdenih odgovora). Ovo možete promeniti ako želite razgovore u nitima.

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Allow 3 levels of nesting
});
[inline-code-end]

### Interna konfiguracija

Ove opcije postavlja automatski Collab Chat i ne bi trebalo da budu zamenjene:

`productId` se automatski postavlja na `3` za Collab Chat. `floating-chat` ekstenzija se automatski učitava kako bi obezbedila funkcionalnost prozora za ćaskanje. Vidžet automatski detektuje mobilne uređaje (ekrani širi manje od 768px) i prilagođava UI u skladu s tim.

### Kompletan primer

Evo primera koji prikazuje više konfiguracionih opcija zajedno:

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
        // Your SSO config
    },
    maxReplyDepth: 1
});
[inline-code-end]

Za kompletnu listu svih dostupnih konfiguracionih opcija nasleđenih iz osnovnog vidžeta, pogledajte glavnu FastComments dokumentaciju o konfiguraciji.