### Overview

FastComments Image Chat breidt de standaard FastComments-reactiewidget uit, waardoor het alle configuratieopties van het basiswidget erft en enkele specifieke opties voor afbeeldingsannotaties toevoegt.

### Required Configuration

#### tenantId

Je FastComments Tenant ID is vereist. Dit vind je in je [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Image Chat Specific Options

#### urlId

Standaard genereert Image Chat een unieke identifier voor elk gesprek op basis van de paginabestand, de afbeeldingsbron en de X/Y-coördinaten. Je kunt dit overschrijven met een aangepaste `urlId`.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Dit is nuttig wanneer je URL-structuur kan veranderen maar je dezelfde gesprekken wilt behouden, of wanneer je annotaties over meerdere pagina's wilt delen.

#### chatSquarePercentage

Bepaalt de grootte van de klikbare chatmarkeringen als percentage van de afbeeldingsbreedte. De standaardwaarde is 5%, wat betekent dat elke markering 5% van de afbeeldingsbreedte is.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Groter, beter zichtbare markeringen
});
```

Kleinere waarden creëren minder opdringerige markeringen die beter werken voor gedetailleerde afbeeldingen. Grotere waarden maken markeringen gemakkelijker te zien en aan te klikken op drukke afbeeldingen of voor gebruikers op mobiele apparaten.

#### hasDarkBackground

Schakel dark mode-styling in wanneer je pagina een donkere achtergrond heeft.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Een callback-functie die wordt aangeroepen telkens wanneer het aantal opmerkingen verandert. Dit is handig voor het bijwerken van UI-elementen zoals badges of paginatitels.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Inherited Configuration Options

Aangezien Image Chat het standaard reacties-widget uitbreidt, kun je elke configuratieoptie van het basis FastComments-widget gebruiken. Hieronder enkele veelgebruikte opties:

#### locale

Stel de taal in voor de widget-UI. FastComments ondersteunt tientallen talen.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Spaans
});
```

#### readonly

Maak alle conversaties alleen-lezen. Gebruikers kunnen bestaande markeringen en discussies bekijken maar geen nieuwe maken of reageren.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso and simpleSSO

Integreer met je authenticatiesysteem met Single Sign-On.

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

Beheer hoeveel niveaus diep reacties kunnen gaan. Standaard zet Image Chat dit op 0, wat betekent dat alle opmerkingen vlak zijn (geen geneste antwoorden). Je kunt dit wijzigen als je geneste conversaties wilt.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Sta 3 niveaus van nesteling toe
});
```

### Internal Configuration

Deze opties worden automatisch door Image Chat ingesteld en mogen niet worden overschreven:

De `productId` is automatisch ingesteld op `2` voor Image Chat. De extensie `floating-chat` wordt automatisch geladen om de chatvensterfunctionaliteit te bieden. Het widget detecteert automatisch mobiele apparaten (schermen onder 768px breed) en past de UI dienovereenkomstig aan met fullscreen chatvensters.

### Target Element Flexibility

De eerste parameter van `FastCommentsImageChat` kan ofwel een `<img>`-element direct zijn of een containerelement met een afbeelding erin:

```javascript
// Direct afbeeldingselement
FastCommentsImageChat(document.getElementById('my-image'), config);

// Container met afbeelding erin
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

Het widget vindt de afbeelding automatisch wanneer je een containerelement doorgeeft.

### Complete Example

Hier is een voorbeeld waarin meerdere configuratieopties samen worden getoond:

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
        // Je SSO-configuratie
    },
    maxReplyDepth: 1
});
```

Voor een volledige lijst van alle beschikbare configuratieopties die zijn overgenomen van het basiswidget, zie de hoofd FastComments-configuratiedocumentatie.