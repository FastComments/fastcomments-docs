### Overzicht

FastComments Image Chat breidt de standaard FastComments commenting widget uit, dus het erft alle configuratieopties van de basiswidget over en voegt een paar opties specifiek voor afbeeldingsannotaties toe.

### Vereiste Configuratie

#### tenantId

Je FastComments Tenant ID is vereist. Je kunt deze vinden in je [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Image Chat Specifieke Opties

#### urlId

Standaard genereert Image Chat een unieke identificatie voor elk gesprek op basis van de pagina-URL, de afbeeldingsbron en de X/Y-coördinaten. Je kunt dit overschrijven met een aangepaste `urlId`.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Dit is handig wanneer je URL-structuur mogelijk verandert maar je dezelfde gesprekken wilt behouden, of wanneer je annotaties over meerdere pagina's wilt delen.

#### chatSquarePercentage

Bepaalt de grootte van de klikbare chat-markers als een percentage van de afbeeldingsbreedte. De standaardwaarde is 5%, wat betekent dat elke marker 5% van de afbeeldingsbreedte is.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Grotere, beter zichtbare markers
});
```

Kleinere waarden creëren minder opvallende markers die beter werken voor gedetailleerde afbeeldingen. Grotere waarden maken markers makkelijker te zien en aan te klikken op drukke afbeeldingen of voor gebruikers op mobiele apparaten.

#### hasDarkBackground

Activeer donkere modus-styling wanneer je pagina een donkere achtergrond heeft.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Een callback-functie die wordt aangeroepen wanneer het aantal reacties verandert. Dit is handig voor het bijwerken van UI-elementen zoals badges of paginatitels.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Overgeërfde Configuratieopties

Omdat Image Chat de standaard commenting widget uitbreidt, kun je elke configuratieoptie van de basis FastComments widget gebruiken. Hier zijn enkele veelgebruikte opties:

#### locale

Stel de taal in voor de widget-UI. FastComments ondersteunt tientallen talen.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Spaans
});
```

#### readonly

Maak alle gesprekken alleen-lezen. Gebruikers kunnen bestaande markers en discussies bekijken, maar kunnen geen nieuwe maken of reageren.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso en simpleSSO

Integreer met je authenticatiesysteem met behulp van Single Sign-On.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // SSO-configuratie
    }
});
```

Zie de SSO-documentatie voor volledige details over authenticatieopties.

#### maxReplyDepth

Bepaal hoeveel niveaus diep reacties kunnen gaan. Standaard stelt Image Chat dit in op 0, wat betekent dat alle reacties vlak zijn (geen geneste reacties). Je kunt dit wijzigen als je gesprekken met threads wilt.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Sta 3 niveaus van nesting toe
});
```

### Interne Configuratie

Deze opties worden automatisch ingesteld door Image Chat en moeten niet worden overschreven:

De `productId` wordt automatisch ingesteld op `2` voor Image Chat. De `floating-chat` extensie wordt automatisch geladen om de chatvenster-functionaliteit te bieden. De widget detecteert automatisch mobiele apparaten (schermen smaller dan 768px) en past de UI dienovereenkomstig aan met fullscreen chatvensters.

### Flexibiliteit van Doel-element

De eerste parameter van `FastCommentsImageChat` kan direct een `<img>`-element zijn of een containerelement met een afbeelding erin:

```javascript
// Direct afbeeldingselement
FastCommentsImageChat(document.getElementById('my-image'), config);

// Container met afbeelding erin
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

De widget vindt de afbeelding automatisch als je een containerelement doorgeeft.

### Volledig Voorbeeld

Hier is een voorbeeld dat meerdere configuratieopties samen toont:

```javascript
FastCommentsImageChat(document.getElementById('product-image'), {
    tenantId: 'demo',
    urlId: 'product-v2-main',
    chatSquarePercentage: 6,
    hasDarkBackground: false,
    locale: 'en',
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) Product Photo` : 'Product Photo';
    },
    sso: {
        // Jouw SSO-configuratie
    },
    maxReplyDepth: 1
});
```

Zie de hoofddocumentatie van FastComments-configuratie voor een volledige lijst van alle beschikbare configuratieopties die van de basiswidget worden overgeërfd.