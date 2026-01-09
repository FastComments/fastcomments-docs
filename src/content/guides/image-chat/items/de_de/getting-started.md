### Anwendungsfälle

Image Chat eignet sich hervorragend für Design-Feedback, bei dem Teams bestimmte Elemente in Mockups oder Screenshots diskutieren müssen. Produktbewertungsseiten können Kunden ermöglichen, über spezifische Merkmale auf Produktfotos zu diskutieren. Bildungsplattformen können es zur Besprechung von Diagrammen, Karten oder wissenschaftlichen Bildern verwenden. Fotogalerien können mit standortbezogenen Kommentaren interaktiv werden. Immobilienseiten können Betrachtern erlauben, bestimmte Merkmale auf Objektfotos zu diskutieren.

### Schnellstart

Der Einstieg in Image Chat ist einfach. Sie benötigen das FastComments Image Chat-Skript, ein Image-Element oder einen Container mit einem Bild und ein Konfigurationsobjekt mit Ihrer Tenant ID.

### Installation

Fügen Sie das Image Chat-Skript zu Ihrer Seite hinzu:

[inline-code-attrs-start title = 'Laden des Image Chat-Skripts'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
[inline-code-end]

### Grundlegende Implementierung

Hier ist ein minimales Beispiel:

[inline-code-attrs-start title = 'Grundlegende Image Chat-Implementierung'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Image Gallery with Image Chat</title>
</head>
<body>
    <!-- Ihr Bild -->
    <img id="my-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Pretty Trail" />

    <!-- Load the Image Chat script -->
    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>

    <!-- Image Chat initialisieren -->
    <script>
        FastCommentsImageChat(document.getElementById('my-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Ersetzen Sie 'demo' durch Ihre tatsächliche FastComments Tenant ID, falls diese nicht bereits korrekt ist. Sie finden sie in Ihrem [FastComments-Dashboard](https://fastcomments.com/auth/my-account).

### Funktionsweise

Sobald Image Chat initialisiert ist, können Benutzer überall auf das Bild klicken. Wenn ein Klick erfolgt, erscheint an dieser Stelle ein sichtbarer quadratischer Marker und ein Chatfenster öffnet sich. Andere Benutzer können alle Marker sehen und darauf klicken, um diese Diskussionen anzusehen oder daran teilzunehmen. Alle Diskussionen werden in Echtzeit über alle Besucher hinweg synchronisiert.

Das Widget verwendet prozentbasierte Positionierung, sodass die Marker an der richtigen Stelle bleiben, selbst wenn sich die Bildgröße ändert oder das Bild auf unterschiedlichen Bildschirmgrößen angezeigt wird.

### Live-Demo

Sie können Image Chat in Aktion auf unserer [Live-Demo-Seite](https://fastcomments.com/product/image-chat) sehen.

### Nächste Schritte

Nachdem Sie die Grundlagen eingerichtet haben, können Sie das Aussehen und Verhalten in der Anleitung zu Konfigurationsoptionen anpassen. Sehen Sie sich die Anleitung zum Responsive Design an, um zu verstehen, wie die prozentbasierte Positionierung funktioniert. Erfahren Sie mehr über Styling und Unterstützung für den Dunkelmodus in der Anpassungsanleitung. Für fortgeschrittene Integrationen sehen Sie sich das API-Referenzhandbuch an.

### Frontend-Bibliotheken

Alle Frontend-Bibliotheken von FastComments (react, vue, angular usw.) bieten Image Chat.