### Schnellstart

Der Einstieg in Collab Chat ist einfach. Sie benötigen das FastComments Collab Chat Skript, ein HTML-Element mit dem Text, den Sie annotieren möchten, und ein Konfigurationsobjekt mit Ihrer Tenant ID.

### Installation

Fügen Sie das Collab Chat Skript zu Ihrer Seite hinzu:

[inline-code-attrs-start title = 'Laden des Collab Chat Skripts'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### Grundlegende Implementierung

Hier ist ein minimales Beispiel:

[inline-code-attrs-start title = 'Grundlegende Collab Chat Implementierung'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

Ersetzen Sie `'demo'` durch Ihre tatsächliche FastComments Tenant ID, falls dies noch nicht geschehen ist. Diese finden Sie in Ihrem [FastComments Dashboard](https://fastcomments.com/auth/my-account/api-secret).

### Funktionsweise

Nach der Initialisierung können Benutzer beliebigen Text innerhalb des Zielelements auswählen. Nach einer kurzen Verzögerung (3,5 Sekunden auf dem Desktop) erscheint eine Eingabeaufforderung, mit der sie eine Diskussion starten können. Wenn eine Diskussion erstellt wird, erscheint eine visuelle Hervorhebung auf dem Text. Andere Benutzer können mit der Maus über die Hervorhebung fahren oder darauf klicken, um die Diskussion anzusehen und daran teilzunehmen. Alle Diskussionen werden in Echtzeit über alle Besucher hinweg synchronisiert.

### Live-Demo

Sie können Collab Chat in Aktion auf unserer [Live-Demo-Seite](https://fastcomments.com/product/collab-chat) sehen.

### Nächste Schritte

Jetzt, da die Grundlagen funktionieren, können Sie das Erscheinungsbild und Verhalten im Configuration Options Guide anpassen. Sehen Sie sich den Text Selection Behavior Guide an, um zu verstehen, wie die Textauswahl funktioniert. Erfahren Sie mehr über Styling und Dark-Mode-Unterstützung im Customization Guide. Für fortgeschrittene Integrationen erkunden Sie die API Reference.

### Frontend-Bibliotheken

Alle FastComments Frontend-Bibliotheken (react, vue, angular usw.) enthalten Collab Chat.