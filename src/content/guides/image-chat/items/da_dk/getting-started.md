### Anvendelsestilfælde

Image Chat fungerer godt til designfeedback, hvor teams skal diskutere specifikke elementer i mockups eller skærmbilleder. Produkttestsites kan lade kunder diskutere specifikke funktioner synlige på produktfotos. Uddannelsesplatforme kan bruge det til at diskutere diagrammer, kort eller videnskabelige billeder. Fotogallerier kan blive interaktive med lokationsspecifikke kommentarer. Ejendomssites kan lade besøgende diskutere specifikke træk synlige på boligbilleder.

### Kom godt i gang

Det er enkelt at komme i gang med Image Chat. Du skal bruge FastComments Image Chat-scriptet, et billedelement eller en container med et billede og et konfigurationsobjekt med dit Tenant ID.

### Installation

Tilføj Image Chat-scriptet til din side:

[inline-code-attrs-start title = 'Indlæs Image Chat-scriptet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
[inline-code-end]

### Grundlæggende implementering

Here's a minimal example:

[inline-code-attrs-start title = 'Grundlæggende Image Chat-implementering'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

Erstat `'demo'` med dit faktiske FastComments Tenant ID, hvis det ikke allerede er, som du kan finde i dit [FastComments dashboard](https://fastcomments.com/auth/my-account).

### Hvordan det virker

Når det er initialiseret, kan brugere klikke hvor som helst på billedet. Når der klikkes, vises en visuel firkantet markør på det sted, og et chatvindue åbnes. Andre brugere kan se alle markørerne og klikke på dem for at se eller deltage i de pågældende diskussioner. Alle diskussioner synkroniseres i realtid på tværs af alle besøgende.

Widget'en bruger procentbaseret placering, så markørerne bliver i den korrekte position, selv når billedet ændrer størrelse eller vises på forskellige skærmstørrelser.

### Live-demo

Du kan se Image Chat i aktion på vores [live demo-side](https://fastcomments.com/product/image-chat).

### Næste skridt

Nu hvor du har det grundlæggende til at fungere, kan du tilpasse udseendet og adfærden i vejledningen om konfigurationsmuligheder. Tjek vejledningen om responsivt design for at forstå, hvordan procentbaseret placering fungerer. Lær om styling og understøttelse af dark mode i tilpasningsvejledningen. For avancerede integrationer kan du udforske API-referencen.

### Frontend-biblioteker

Alle FastComments frontend-biblioteker (react, vue, angular osv.) indeholder Image Chat.