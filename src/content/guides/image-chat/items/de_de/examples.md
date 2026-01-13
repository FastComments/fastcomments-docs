### Einfaches Beispiel

Die einfachste Möglichkeit, Image Chat zu verwenden, besteht darin, ein einzelnes Image-Element anzusprechen. Dieses Beispiel zeigt, wie man interaktive Diskussionen für ein Bild aktiviert:

[inline-code-attrs-start title = 'Einfaches Image-Chat-Beispiel'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>Product Image with Chat</title>
</head>
<body>
    <img id="product-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Product Photo" />

    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
    <script>
        FastCommentsImageChat(document.getElementById('product-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

### Beispiel mit Container-Element

Sie können auch ein Container-Element übergeben, das ein Bild enthält:

[inline-code-attrs-start title = 'Image-Chat mit Container'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<div id="image-container">
    <img src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="System Diagram" />
</div>

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('image-container'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

### Beispiel mit benutzerdefinierter URL-ID

Standardmäßig verwendet Image Chat die Seiten-URL kombiniert mit der Bildquelle und den Koordinaten, um Unterhaltungen zu identifizieren. Sie können eine benutzerdefinierte `urlId` angeben:

[inline-code-attrs-start title = 'Image-Chat mit benutzerdefinierter URL-ID'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        urlId: 'product-v2-main-image'
    });
</script>
[inline-code-end]

Das ist nützlich, wenn sich Ihre URL-Struktur ändert, Sie aber dieselben Unterhaltungen beibehalten möchten, oder wenn Sie dieselben Diskussionspunkte über mehrere Seiten teilen möchten.

### Beispiel mit Dunkelmodus

Falls Ihre Seite einen dunklen Hintergrund hat und das Widget diesen nicht automatisch erkennt wie es sollte, können wir die Unterstützung für den Dunkelmodus manuell aktivieren:

[inline-code-attrs-start title = 'Image-Chat mit Dunkelmodus'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

### Beispiel mit benutzerdefinierter Chat-Quadratgröße

Sie können die Größe der anklickbaren Quadrate, die auf dem Bild erscheinen, anpassen. Die Größe wird als Prozentsatz der Bildbreite angegeben:

[inline-code-attrs-start title = 'Image-Chat mit benutzerdefinierter Quadratgröße'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>Image Chat with Custom Square Size</title>
</head>
<body>
    <img id="product-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Product Photo" />

    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
    <script>
        FastCommentsImageChat(document.getElementById('product-image'), {
            tenantId: 'demo',
            chatSquarePercentage: 2, // Kleinere Quadrate (Standard ist 5)
        });
    </script>
</body>
</html>
[inline-code-end]

### Beispiel mit Kommentaranzahl-Callback

Verfolgen Sie, wann Kommentare hinzugefügt oder aktualisiert werden, mithilfe des `commentCountUpdated`-Callbacks:

[inline-code-attrs-start title = 'Image-Chat mit Kommentaranzahl-Callback'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        commentCountUpdated: function(count) {
            console.log('Total comments:', count);
            document.getElementById('comment-badge').textContent = count;
        }
    });
</script>
[inline-code-end]

### Beispiel mit mehreren Bildern

Sie können Image Chat auf mehreren Bildern initialisieren. Jedes Bild hat seine eigenen unabhängigen Diskussionspunkte:

[inline-code-attrs-start title = 'Image-Chat auf mehreren Bildern'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<img id="image-1" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 1" />
<img id="image-2" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 2" />

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    // Auf dem ersten Bild initialisieren
    FastCommentsImageChat(document.getElementById('image-1'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-1'
    });

    // Auf dem zweiten Bild initialisieren
    FastCommentsImageChat(document.getElementById('image-2'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-2'
    });
</script>
[inline-code-end]

---