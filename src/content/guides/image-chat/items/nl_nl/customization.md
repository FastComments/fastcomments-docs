### Ondersteuning voor donkere modus

Image Chat bevat ingebouwde ondersteuning voor donkere modus. Als u `hasDarkBackground: true` instelt in uw configuratie, passen de chatvensters en UI-elementen zich automatisch aan zodat ze goed werken op donkere achtergronden.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

De styling voor donkere modus geldt voor chatvensters, marker-vierkanten en alle interactieve elementen. Als uw site een schakelaar voor donkere modus heeft, kunt u de widget opnieuw initialiseren wanneer de modus verandert, of de body-class methode gebruiken die hieronder wordt beschreven.

### Dynamische donkere modus

Als de donkere modus van uw site wordt geregeld door een `.dark`-klasse toe te voegen aan het body-element, zal de Image Chat UI dit automatisch respecteren zonder dat herinitialisatie nodig is. De stijlen van de widget zijn ontworpen om te reageren op de aanwezigheid van deze klasse.

```css
/* Uw CSS voor donkere modus */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
```

### Aangepaste styling met CSS

U kunt het uiterlijk van markers, chatvensters en andere elementen aanpassen met CSS. De widget voegt specifieke klassen toe die u in uw stylesheet kunt targeten.

De chatvierkanten en vensters gebruiken het FastComments comment bubble-stylingsysteem, dus eventuele aanpassingen die u op de standaard commentaarwidget hebt toegepast, beïnvloeden ook Image Chat.

### Grootte van chatvierkanten

De optie `chatSquarePercentage` regelt de grootte van de aanklikbare markers. Standaard is dit 5% van de afbeeldingsbreedte:

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 7  // Grotere, beter zichtbare vierkanten
});
```

Kleinere waarden maken subtielere markers die in de afbeelding opgaan. Grotere waarden maken markers prominenter en gemakkelijker aan te klikken, vooral op mobiele apparaten of voor toegankelijkheidsdoeleinden.

### Gedrag op mobiel

Op schermen smaller dan 768px schakelt Image Chat automatisch over naar een voor mobiel geoptimaliseerde lay-out. Chatvensters verschijnen in volledig scherm in plaats van naast de markers te zweven, wat betere bruikbaarheid op kleine schermen biedt.

De markers blijven zichtbaar op hun responsieve posities op de afbeelding. Gebruikers kunnen op een marker tikken om de chatinterface in volledig scherm te openen. Dit gedrag is ingebouwd en vereist geen configuratie.

### Uiterlijk van het chatvenster

Chatvensters zijn 300px breed op desktop met een pijl van 16px die naar de marker wijst. De vensters positioneren zichzelf automatisch op basis van de beschikbare viewport-ruimte, met positioneringsklassen zoals `to-right`, `to-left`, `to-top` en `to-bottom`.

U kunt aangepaste CSS toevoegen om kleuren, lettertypen, ruimte of andere visuele eigenschappen van deze vensters aan te passen. De chatvensters gebruiken dezelfde componentstructuur als de standaard FastComments-widget, dus ze erven eventuele globale aanpassingen die u hebt toegepast.

### Luie initialisatie

Chatvensters initialiseren bij hover voor desktopgebruikers of onmiddellijk wanneer ze worden aangemaakt. Dit vermindert de initiële laadtijd doordat de chatinterface alleen wordt gerenderd wanneer gebruikers daadwerkelijk met een marker interageren.

De luie initialisatie gebeurt transparant. Gebruikers merken geen vertraging, maar de browser hoeft niet tientallen verborgen chatvensters te renderen als u veel markers op een afbeelding hebt.

### Lokalisatie

Image Chat ondersteunt alle dezelfde lokalisatie-opties als de standaard FastComments-widget. Stel de `locale`-optie in om UI-tekst in verschillende talen weer te geven:

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'fr'  // Frans
});
```

FastComments ondersteunt tientallen talen. De locale-instelling beïnvloedt alle UI-tekst, inclusief prompts, knoppen en placeholder-tekst.

### Geërfde aanpassingsopties

Omdat Image Chat de standaard commentaarwidget uitbreidt, erft het alle aanpassingsopties van de basiswidget. Dit omvat aangepaste CSS-klassen, aangepaste vertalingen, avatar-aanpassing, datumopmaak en nog veel meer.

Zie de hoofddocumentatie van FastComments voor aanpassingen voor de volledige lijst met beschikbare aanpassingsopties.

### Werken met aangepaste lettertypen

Als uw site aangepaste lettertypen gebruikt, erft de Image Chat UI die lettertypen uit de CSS van uw pagina. De chatvensters worden gerenderd binnen de DOM van uw pagina en respecteren uw bestaande typografie-instellingen.

Voor het beste resultaat zorgt u ervoor dat uw aangepaste lettertypen zijn geladen voordat u Image Chat initialiseert, of accepteert u dat er een korte flash van ongestylede tekst kan optreden terwijl de lettertypen laden.

### Visueel ontwerp van markers

De vierkante markers hebben een subtiel visueel ontwerp dat ze merkbaar maakt zonder de afbeelding te overheersen. U kunt hun uiterlijk aanpassen met CSS als u een andere visuele behandeling wilt.

De markers bevatten hover-states die feedback geven wanneer gebruikers hun muis erover bewegen. Op aanraakapparaten geeft de tikinteractie directe feedback door het chatvenster te openen.