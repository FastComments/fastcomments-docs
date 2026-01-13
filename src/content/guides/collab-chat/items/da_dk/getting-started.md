### Kom godt i gang

Det er enkelt at komme i gang med Collab Chat. Du skal bruge FastComments Collab Chat-scriptet, et HTML-element, der indeholder den tekst, du vil annotere, og et konfigurationsobjekt med dit Tenant ID.

### Installation

Tilføj Collab Chat-scriptet til din side:

[inline-code-attrs-start title = 'Indlæsning af Collab Chat-scriptet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### Grundlæggende implementering

Her er et minimalt eksempel:

[inline-code-attrs-start title = 'Grundlæggende implementering af Collab Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <!-- Your content container -->
    <div id="article-content" style="min-height: 500px;">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <!-- Load the Collab Chat script -->
    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>

    <!-- Initialize Collab Chat -->
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Udskift `'demo'` med dit faktiske FastComments Tenant ID, hvis det ikke allerede er det. Du kan finde det i dit [FastComments-dashboard](https://fastcomments.com/auth/my-account/api-secret).

### Sådan fungerer det

Når det er initialiseret, kan brugere markere enhver tekst inden for mål-elementet. Efter en kort forsinkelse (3,5 sekunder på desktop) vises en prompt, der gør det muligt for dem at starte en diskussion. Når en diskussion oprettes, vises en visuel markering på teksten. Andre brugere kan holde musen over eller klikke på markeringen for at se og deltage i diskussionen. Alle diskussioner synkroniseres i realtid mellem alle besøgende.

### Live-demo

Du kan se Collab Chat i aktion på vores [live demo-side](https://fastcomments.com/product/collab-chat).

### Næste skridt

Nu hvor du har det grundlæggende til at fungere, kan du tilpasse udseende og adfærd i guiden Konfigurationsmuligheder. Se guiden om tekstmarkering for at forstå, hvordan tekstmarkering fungerer. Læs om styling og understøttelse af mørk tilstand i Tilpasningsguiden. For avancerede integrationer kan du udforske API-Referencen.

### Frontend-biblioteker

Alle FastComments frontend-biblioteker (react, vue, angular, etc) har Collab Chat.