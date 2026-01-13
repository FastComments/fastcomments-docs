### Gebruiksscenario's

Image Chat is ideaal voor designfeedback waarbij teams specifieke elementen in mockups of schermafbeeldingen moeten bespreken. Productreviewsites kunnen klanten laten discussiëren over specifieke functies die zichtbaar zijn op productfoto's. Educatieve platforms kunnen het gebruiken om diagrammen, kaarten of wetenschappelijke afbeeldingen te bespreken. Fotogalerijen kunnen interactief worden met locatie-specifieke opmerkingen. Vastgoedsites kunnen kijkers laten praten over specifieke kenmerken die zichtbaar zijn op woningfoto's.

### Snel aan de slag

Aan de slag gaan met Image Chat is eenvoudig. Je hebt het FastComments Image Chat-script, een image-element of container met een afbeelding, en een configuratieobject met je Tenant ID nodig.

### Installatie

Voeg het Image Chat-script toe aan je pagina:

[inline-code-attrs-start title = 'Image Chat-script laden'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
[inline-code-end]

### Basisimplementatie

Hier is een minimaal voorbeeld:

[inline-code-attrs-start title = 'Basisimplementatie van Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Image Gallery with Image Chat</title>
</head>
<body>
    <!-- Your image -->
    <img id="my-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Pretty Trail" />

    <!-- Load the Image Chat script -->
    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>

    <!-- Initialize Image Chat -->
    <script>
        FastCommentsImageChat(document.getElementById('my-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Vervang `'demo'` door je daadwerkelijke FastComments Tenant ID als dat nog niet het geval is; die vind je in je [FastComments dashboard](https://fastcomments.com/auth/my-account).

### Hoe het werkt

Zodra het geïnitialiseerd is, kunnen gebruikers overal op de afbeelding klikken. Wanneer er geklikt wordt, verschijnt er op die locatie een visuele vierkante markering en wordt er een chatvenster geopend. Andere gebruikers kunnen alle markeringen zien en erop klikken om die discussies te bekijken of eraan deel te nemen. Alle discussies worden in realtime gesynchroniseerd voor alle bezoekers.

De widget gebruikt positionering op basis van percentages, zodat markeringen op de juiste plaats blijven, zelfs wanneer de afbeelding van formaat verandert of op verschillende schermformaten wordt bekeken.

### Live-demo

Je kunt Image Chat in actie zien op onze [live demo page](https://fastcomments.com/product/image-chat).

### Volgende stappen

Nu de basis werkt, kun je het uiterlijk en gedrag aanpassen in de gids Configuratie-opties. Bekijk de gids Responsief ontwerp om te begrijpen hoe positionering op basis van percentages werkt. Lees over styling en ondersteuning voor donkere modus in de gids Aanpassing. Voor geavanceerde integraties, bekijk de API-referentie.

### Frontend-bibliotheken

Alle FastComments-frontendbibliotheken (react, vue, angular, etc) bevatten Image Chat.