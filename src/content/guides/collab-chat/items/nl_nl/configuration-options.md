### Overzicht

FastComments Collab Chat breidt de standaard FastComments-commentaarwidget uit, dus het erft alle configuratieopties van de basiswidget en voegt enkele opties toe die specifiek zijn voor tekstannotaties.

### Vereiste configuratie

#### tenantId

Uw FastComments Tenant ID is verplicht. U kunt dit vinden in uw [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Configuratievoorbeeld"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Collab Chat-specifieke opties

#### urlId

Standaard genereert Collab Chat een unieke identificatie voor elk gesprek op basis van de pagina-URL, het DOM-pad naar het element en het geselecteerde tekstdbereik. U kunt dit overschrijven met een aangepaste `urlId`.

[inline-code-attrs-start title = "Configuratievoorbeeld"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Dit is handig wanneer uw URL-structuur kan veranderen maar u dezelfde gesprekken wilt behouden, of wanneer u annotaties over meerdere pagina's wilt delen.

#### topBarTarget

Regelt de weergave van de bovenste balk die het aantal gebruikers en discussies toont. Stel in op `null` om de bovenste balk volledig uit te schakelen, of geef een DOM-element op om deze op een specifieke locatie te renderen.

[inline-code-attrs-start title = "Configuratievoorbeeld"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Topbalk uitschakelen
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Topbalk weergeven op aangepaste locatie
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

Schakel donkere modus-styling in wanneer uw pagina een donkere achtergrond heeft. Deze detectie is automatisch, maar het kan wenselijk zijn om deze te overschrijven.

[inline-code-attrs-start title = "Configuratievoorbeeld"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Een callbackfunctie die wordt aangeroepen telkens wanneer het aantal opmerkingen verandert. Dit is handig voor het bijwerken van UI-elementen zoals badges of paginatitels.

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

### Geërfde configuratie-opties

Aangezien Collab Chat de standaardcommentaarwidget uitbreidt, kunt u elke configuratieoptie van de basis FastComments-widget gebruiken. Hier zijn enkele vaak gebruikte opties:

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

Maak alle conversaties alleen-lezen. Gebruikers kunnen bestaande annotaties bekijken maar geen nieuwe maken of reageren.

[inline-code-attrs-start title = "Configuratievoorbeeld"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso and simpleSSO

Integreer met uw authenticatiesysteem met Single Sign-On.

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

Beheer hoe diep replies genest mogen zijn. Standaard stelt Collab Chat dit in op 0, wat betekent dat alle opmerkingen vlak zijn (geen geneste antwoorden). U kunt dit wijzigen als u draadgesprekken wilt.

[inline-code-attrs-start title = "Configuratievoorbeeld"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Sta 3 niveaus van geneste reacties toe
});
[inline-code-end]

### Interne configuratie

Deze opties worden automatisch ingesteld door Collab Chat en mogen niet worden overschreven:

De `productId` wordt automatisch ingesteld op `3` voor Collab Chat. De extensie `floating-chat` wordt automatisch geladen om de chatvensterfunctionaliteit te bieden. De widget detecteert automatisch mobiele apparaten (schermen smaller dan 768px) en past de UI dienovereenkomstig aan.

### Volledig voorbeeld

Hier is een voorbeeld waarin meerdere configuratieopties samen worden getoond:

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
        // Uw SSO-configuratie
    },
    maxReplyDepth: 1
});
[inline-code-end]

Voor een volledige lijst van alle beschikbare configuratieopties die worden geërfd van de basiswidget, zie de hoofd FastComments-configuratiedocumentatie.

---