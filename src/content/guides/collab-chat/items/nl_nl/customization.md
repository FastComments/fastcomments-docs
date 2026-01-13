### Donkere modus-ondersteuning

### Dynamische donkere modus

Als de donkere modus van uw site wordt geregeld door een `.dark`-klasse aan het body-element toe te voegen, respecteert de Collab Chat UI dit automatisch zonder herinitialisatie. De stijlen van de widget zijn zo ontworpen dat ze reageren op de aanwezigheid van deze klasse.

[inline-code-attrs-start title = 'Voorbeeld CSS voor donkere modus'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/* Uw CSS voor donkere modus */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
[inline-code-end]

### Aangepaste styling met CSS

U kunt het uiterlijk van markeringen, chatvensters en andere elementen aanpassen met CSS. De widget voegt specifieke klassen toe die u in uw stylesheet kunt targeten.

Tekstmarkeringen gebruiken het FastComments-stijlsysteem voor commentbubbels, dus eventuele aanpassingen die u op de standaard commentaarwidget hebt toegepast, hebben ook invloed op Collab Chat.

### Aanpassen van de bovenbalk

De bovenbalk toont het aantal gebruikers online en het aantal discussies. U kunt de positie aanpassen door een aangepast element op te geven als `topBarTarget`:

[inline-code-attrs-start title = 'Aangepaste locatie voor bovenbalk'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('my-custom-header')
});
[inline-code-end]

Of schakel het volledig uit door het op `null` te zetten:

[inline-code-attrs-start title = 'Bovenbalk uitschakelen'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});
[inline-code-end]

### Mobiel gedrag

Op schermen die smaller zijn dan 768px schakelt Collab Chat automatisch over naar een mobiel geoptimaliseerde lay-out. Chatvensters verschijnen fullscreen in plaats van zwevend naast de tekst, en de selectievertraging wordt verwijderd voor directere interactie.

Dit gedrag is ingebouwd en vereist geen configuratie. De widget detecteert automatisch de schermgrootte en past zich dienovereenkomstig aan.

### Uiterlijk van chatvensters

Chatvensters zijn op desktop 410px breed met een pijl van 16px die naar de gemarkeerde tekst wijst. De vensters positioneren zichzelf automatisch op basis van de beschikbare viewport-ruimte, waarbij positioneringsklassen zoals `to-right`, `to-left`, `to-top` en `to-bottom` worden gebruikt.

U kunt aangepaste CSS toevoegen om kleuren, lettertypen, afstand of andere visuele eigenschappen van deze vensters aan te passen. De chatvensters gebruiken dezelfde componentstructuur als de standaard FastComments-widget, dus ze erven eventuele globale aanpassingen die u hebt toegepast.

### Lokalisatie

Collab Chat ondersteunt alle dezelfde lokalisatie-opties als de standaard FastComments-widget. Stel de optie `locale` in om UI-tekst in andere talen weer te geven:

[inline-code-attrs-start title = 'Locale instellen'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Spaans
});
[inline-code-end]

FastComments ondersteunt tientallen talen. De locale-instelling beïnvloedt alle UI-tekst, inclusief prompts, knoppen en placeholder-tekst.

### Geërfde aanpassingsopties

Aangezien Collab Chat het standaard commentaarwidget uitbreidt, erft het alle aanpassingsopties van het basiswidget. Dit omvat aangepaste CSS-klassen, aangepaste vertalingen, avatar-aanpassing, datumopmaak en nog veel meer.

Zie de hoofddocumentatie van FastComments voor een volledige lijst met beschikbare aanpassingsopties.

### Werken met aangepaste lettertypen

Als uw site aangepaste lettertypen gebruikt, erft de Collab Chat UI die lettertypen van de CSS van uw pagina. U moet mogelijk een widget-aanpassingsregel maken en `@import` eventuele lettertypen in de aangepaste CSS in die regel als u
wilt dat het zwevende chatvenster dezelfde lettertypen gebruikt.