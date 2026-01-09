### Snel aan de slag

Aan de slag met Collab Chat is eenvoudig. U heeft het FastComments Collab Chat-script nodig, een HTML-element dat de tekst bevat die u wilt annoteren, en een configuratieobject met uw Tenant ID.

### Installatie

Voeg het Collab Chat-script aan uw pagina toe:

[inline-code-attrs-start title = 'Collab Chat-script laden'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### Basisimplementatie

Hier is een minimaal voorbeeld:

[inline-code-attrs-start title = 'Basisimplementatie van Collab Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <!-- Uw inhoudcontainer -->
    <div id="article-content" style="min-height: 500px;">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <!-- Laad het Collab Chat-script -->
    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>

    <!-- Initialiseer Collab Chat -->
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Vervang `'demo'` door uw werkelijke FastComments Tenant ID als dat nog niet het geval is; deze vindt u in uw [FastComments-dashboard](https://fastcomments.com/auth/my-account/api-secret).

### Hoe het werkt

Zodra het is geïnitialiseerd, kunnen gebruikers willekeurige tekst binnen het doel-element selecteren. Na een korte vertraging (3,5 seconden op desktop) verschijnt er een prompt waarmee ze een discussie kunnen starten. Wanneer een discussie wordt aangemaakt, verschijnt er een visuele markering op de tekst. Andere gebruikers kunnen met de muis over de markering zweven of erop klikken om de discussie te bekijken en eraan deel te nemen. Alle discussies synchroniseren in realtime tussen alle bezoekers.

### Live demo

U kunt Collab Chat in actie zien op onze [live demo-pagina](https://fastcomments.com/product/collab-chat).

### Volgende stappen

Nu u de basis werkend heeft, kunt u het uiterlijk en gedrag aanpassen in de gids Configuratieopties. Bekijk de gids Tekstselectiegedrag om te begrijpen hoe tekstselectie werkt. Lees over styling en ondersteuning voor donkere modus in de gids Aanpassing. Voor geavanceerde integraties, bekijk de API-referentie.

### Frontend-bibliotheken

Alle FastComments-frontendbibliotheken (react, vue, angular, etc) hebben Collab Chat.