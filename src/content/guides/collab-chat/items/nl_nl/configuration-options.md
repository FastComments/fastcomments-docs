### Overzicht

FastComments Collab Chat breidt de standaard FastComments commenting widget uit, dus het erft alle configuratieopties van de basiswidget over, terwijl het een paar opties specifiek voor tekstannotaties toevoegt.

### Vereiste Configuratie

#### tenantId

Je FastComments Tenant ID is vereist. Je kunt deze vinden in je [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Configuratievoorbeeld"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Collab Chat Specifieke Opties

#### urlId

Standaard genereert Collab Chat een unieke identifier voor elk gesprek op basis van de pagina-URL, het DOM-pad naar het element en het geselecteerde tekstbereik. Je kunt dit overschrijven met een aangepaste `urlId`.

[inline-code-attrs-start title = "Configuratievoorbeeld"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Dit is handig wanneer je URL-structuur mogelijk verandert maar je dezelfde gesprekken wilt behouden, of wanneer je annotaties over meerdere pagina's wilt delen.

#### topBarTarget

Regelt de weergave van de bovenbalk die het aantal gebruikers en het aantal discussies toont. Stel in op `null` om de bovenbalk volledig uit te schakelen, of geef een DOM-element op om deze op een specifieke locatie weer te geven.

[inline-code-attrs-start title = "Configuratievoorbeeld"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Bovenbalk uitschakelen
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Bovenbalk weergeven op aangepaste locatie
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

Activeer donkere modus-styling wanneer je pagina een donkere achtergrond heeft. Deze detectie is automatisch, maar het kan wenselijk zijn om deze te overschrijven.

[inline-code-attrs-start title = "Configuratievoorbeeld"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Een callback-functie die wordt aangeroepen wanneer het aantal reacties verandert. Dit is handig voor het bijwerken van UI-elementen zoals badges of paginatitels.

[inline-code-attrs-start title = "Configuratievoorbeeld"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### Overgeërfde Configuratieopties

Aangezien Collab Chat de standaard commenting widget uitbreidt, kun je elke configuratieoptie van de basis FastComments widget gebruiken. Hier zijn enkele veelgebruikte opties:

#### locale

Stel de taal in voor de widget-UI. FastComments ondersteunt tientallen talen.

[inline-code-attrs-start title = "Configuratievoorbeeld"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Spaans
});
[inline-code-end]

#### readonly

Maak alle gesprekken alleen-lezen. Gebruikers kunnen bestaande annotaties bekijken, maar geen nieuwe maken of reageren.

[inline-code-attrs-start title = "Configuratievoorbeeld"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso en simpleSSO

Integreer met je authenticatiesysteem met behulp van Single Sign-On.

[inline-code-attrs-start title = "Configuratievoorbeeld"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // SSO-configuratie
    }
});
[inline-code-end]

Zie de SSO-documentatie voor volledige details over authenticatieopties.

#### maxReplyDepth

Regel hoeveel niveaus diep reacties kunnen gaan. Standaard stelt Collab Chat dit in op 0, wat betekent dat alle reacties vlak zijn (geen geneste reacties). Je kunt dit wijzigen als je gegroepeerde gesprekken wilt.

[inline-code-attrs-start title = "Configuratievoorbeeld"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Sta 3 niveaus van nesting toe
});
[inline-code-end]

### Interne Configuratie

Deze opties worden automatisch ingesteld door Collab Chat en moeten niet worden overschreven:

De `productId` wordt automatisch ingesteld op `3` voor Collab Chat. De `floating-chat` extensie wordt automatisch geladen om de chatvensterfunctionaliteit te bieden. De widget detecteert automatisch mobiele apparaten (schermen smaller dan 768px) en past de UI dienovereenkomstig aan.

### Volledig Voorbeeld

Hier is een voorbeeld dat meerdere configuratieopties samen toont:

[inline-code-attrs-start title = "Configuratievoorbeeld"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
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
        // Je SSO-configuratie
    },
    maxReplyDepth: 1
});
[inline-code-end]

Voor een volledige lijst van alle beschikbare configuratieopties die van de basiswidget zijn overgeërfd, zie de hoofddocumentatie van de FastComments configuratie.