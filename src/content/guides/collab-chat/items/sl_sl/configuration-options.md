### Pregled

FastComments Collab Chat razširja standardni widget za komentarje FastComments, zato podeduje vse konfiguracijske možnosti osnovnega widgeta in hkrati doda nekaj možnosti, specifičnih za besedilne pripombe.

### Zahtevana konfiguracija

#### tenantId

Vaš FastComments Tenant ID je obvezen. Najdete ga v svoji [nadzorni plošči FastComments](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Možnosti, specifične za Collab Chat

#### urlId

Privzeto Collab Chat generira edinstven identifikator za vsak pogovor na podlagi URL-ja strani, DOM-poti do elementa in izbranega besedilnega obsega. To lahko preglasite s svojim `urlId`.

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

To je uporabno, kadar se struktura vaših URL-jev lahko spremeni, a želite ohraniti iste pogovore, ali kadar želite pripombe deliti med več stranmi.

#### topBarTarget

Nadzoruje prikaz zgornje vrstice, ki prikazuje število uporabnikov in število razprav. Nastavite na `null`, da zgornjo vrstico popolnoma onemogočite, ali pa podajte DOM element, da jo prikažete na določenem mestu.

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

Omogočite temni način oblikovanja, kadar ima vaša stran temno ozadje. To zaznavanje je samodejno, vendar ga je včasih smiselno pregglasiti.

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Povratna funkcija, ki se sproži vsakič, ko se spremeni število komentarjev. To je uporabno za posodabljanje elementov vmesnika, kot so značke ali naslovi strani.

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

### Podedovane konfiguracijske možnosti

Ker Collab Chat razširja standardni widget za komentarje, lahko uporabite katero koli konfiguracijsko možnost osnovnega widgeta FastComments. Tukaj je nekaj pogosto uporabljenih možnosti:

#### locale

Nastavite jezik za uporabniški vmesnik widgeta. FastComments podpira ducatne jezike.

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Spanish
});
[inline-code-end]

#### readonly

Naredite vse pogovore samo za branje. Uporabniki si lahko ogledajo obstoječe pripombe, ne morejo pa ustvarjati novih ali odgovarjati.

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso in simpleSSO

Integrirajte se s svojim sistemom avtentikacije z uporabo Single Sign-On.

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // SSO configuration
    }
});
[inline-code-end]

Za podrobnosti o možnostih avtentikacije si oglejte dokumentacijo SSO.

#### maxReplyDepth

Nadzorujte, na koliko ravneh globoko lahko gredo odgovori. Privzeto Collab Chat nastavi to vrednost na 0, kar pomeni, da so vsi komentarji ravni (brez ugnezdenih odgovorov). To lahko spremenite, če želite razvejane pogovore.

[inline-code-attrs-start title = "Primer konfiguracije"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Allow 3 levels of nesting
});
[inline-code-end]

### Notranja konfiguracija

Te možnosti nastavi samodejno Collab Chat in jih ne bi smeli pregglasiti:

`productId` je samodejno nastavljen na `3` za Collab Chat. Razširitev `floating-chat` se samodejno naloži za zagotavljanje funkcionalnosti okna klepeta. Widget samodejno zazna mobilne naprave (zaslone širše manj kot 768px) in ustrezno prilagodi uporabniški vmesnik.

### Popoln primer

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
        // Your SSO config
    },
    maxReplyDepth: 1
});
[inline-code-end]

Za popoln seznam vseh razpoložljivih konfiguracijskih možnosti, podedovanih iz osnovnega widgeta, si oglejte glavno dokumentacijo konfiguracije FastComments.