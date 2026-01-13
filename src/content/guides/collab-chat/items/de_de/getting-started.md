### Schnellstart

Der Einstieg in Collab Chat ist einfach. Sie benötigen das FastComments Collab Chat-Skript, ein HTML-Element, das den Text enthält, den Sie annotieren möchten, sowie ein Konfigurationsobjekt mit Ihrer Tenant-ID.

### Installation

Fügen Sie das Collab Chat-Skript zu Ihrer Seite hinzu:

[inline-code-attrs-start title = 'Collab Chat-Skript laden'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### Basic Implementation

Here's a minimal example:

[inline-code-attrs-start title = 'Grundlegende Collab Chat-Implementierung'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <!-- Ihr Inhaltscontainer -->
    <div id="article-content" style="min-height: 500px;">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <!-- Das Collab Chat-Skript laden -->
    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>

    <!-- Collab Chat initialisieren -->
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Ersetzen Sie `'demo'` durch Ihre tatsächliche FastComments Tenant-ID, falls diese nicht bereits korrekt ist. Sie finden sie in Ihrem [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

### How It Works

Sobald die Initialisierung erfolgt ist, können Benutzer beliebigen Text innerhalb des Ziel-Elements auswählen. Nach einer kurzen Verzögerung (3,5 Sekunden auf dem Desktop) erscheint eine Eingabeaufforderung, die es ihnen ermöglicht, eine Diskussion zu starten. Wenn eine Diskussion erstellt wird, erscheint eine visuelle Hervorhebung des Texts. Andere Benutzer können über die Hervorhebung fahren oder klicken, um die Diskussion zu sehen und daran teilzunehmen. Alle Diskussionen werden in Echtzeit für alle Besucher synchronisiert.

### Live Demo

Sie können Collab Chat in Aktion auf unserer [Live-Demo-Seite](https://fastcomments.com/product/collab-chat) sehen.

### Nächste Schritte

Jetzt, wo die Grundlagen funktionieren, können Sie das Aussehen und Verhalten im Leitfaden zu den Konfigurationsoptionen anpassen. Sehen Sie sich den Leitfaden zum Verhalten bei Textauswahl an, um zu verstehen, wie die Textauswahl funktioniert. Erfahren Sie mehr über Styling und Unterstützung für den Dunkelmodus im Anpassungsleitfaden. Für fortgeschrittene Integrationen sehen Sie sich das API Reference an.

### Frontend Libraries

Alle FastComments-Frontend-Bibliotheken (react, vue, angular, etc) enthalten Collab Chat.