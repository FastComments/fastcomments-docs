### Grundlegendes Beispiel

Der einfachste Weg, Collab Chat zu verwenden, besteht darin, einen einzelnen Inhaltscontainer anzusprechen. Dieses Beispiel zeigt, wie man Textannotationen in einem Artikel aktiviert:

[inline-code-attrs-start title = 'Grundlegendes Collab Chat Beispiel'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <div id="article-content" style="min-height: 500px">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

### Beispiel mit benutzerdefinierter URL-ID (pro Buchseite, Artikel usw.)

Standardmäßig verwendet Collab Chat die Seiten-URL kombiniert mit dem Elemente-Pfad und dem Textbereich, um Konversationen zu identifizieren. Sie können eine benutzerdefinierte `urlId` angeben, um mehr Kontrolle darüber zu haben, wie Konversationen gruppiert werden:

[inline-code-attrs-start title = 'Collab Chat mit benutzerdefinierter URL-ID'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    FastCommentsCollabChat(document.getElementById('article-content'), {
        tenantId: 'demo',
        urlId: 'book-one-page-2'
    });
</script>
[inline-code-end]

Das ist nützlich, wenn sich Ihre URL-Struktur ändert, Sie aber dieselben Unterhaltungen beibehalten möchten, oder wenn Sie dieselben Konversationsannotationen über mehrere Seiten hinweg teilen möchten.

### Beispiel mit Dunkelmodus

Wenn Ihre Seite einen dunklen Hintergrund hat, aktivieren Sie die Unterstützung für den Dunkelmodus, damit die Chat-Benutzeroberfläche korrekt dargestellt wird:

[inline-code-attrs-start title = 'Collab Chat mit Dunkelmodus'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat - Dark Mode</title>
    <style>
        body {
            background-color: #1a1a1a !important;
            color: #e0e0e0 !important;
            font-family: system-ui, -apple-system, sans-serif;
            padding: 20px;
            margin: 0;
        }
        #article-content {
            max-width: 800px;
            margin: 0 auto;
            background-color: #2d2d2d;
            padding: 20px;
            border-radius: 8px;
        }
        h1 {
            color: #ffffff !important;
        }
        p {
            color: #e0e0e0 !important;
            line-height: 1.6;
        }
        .fastcomments-collab-chat-top-bar {
            background-color: #2d2d2d !important;
            color: #e0e0e0 !important;
            border-bottom: 1px solid #444 !important;
        }
    </style>
</head>
<body>
    <div id="article-content" style="min-height: 500px">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo',
            hasDarkBackground: true
        });
    </script>
</body>
</html>
[inline-code-end]

### Beispiel mit deaktivierter Top-Leiste

Standardmäßig zeigt Collab Chat eine obere Leiste mit Benutzeranzahl und Diskussionsanzahl an. Sie können sie deaktivieren:

[inline-code-attrs-start title = 'Collab Chat mit deaktivierter Top-Leiste'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat - No Top Bar</title>
</head>
<body>
    <div id="article-content" style="min-height: 500px">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo',
            topBarTarget: null
        });
    </script>
</body>
</html>
[inline-code-end]

### Beispiel mit Callback für Kommentaranzahl

Sie können verfolgen, wann Kommentare hinzugefügt oder aktualisiert werden, indem Sie den Callback `commentCountUpdated` verwenden:

[inline-code-attrs-start title = 'Collab Chat mit Callback für Kommentaranzahl'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    FastCommentsCollabChat(document.getElementById('article-content'), {
        tenantId: 'demo',
        commentCountUpdated: function(count) {
            console.log('Total comments:', count);
            document.getElementById('comment-badge').textContent = count;
        }
    });
</script>
[inline-code-end]

### Beispiel mit mehreren Abschnitten

Sie können Collab Chat in mehreren separaten Abschnitten Ihrer Seite initialisieren. Jeder Abschnitt hat seine eigenen unabhängigen Annotationen:

[inline-code-attrs-start title = 'Collab Chat auf mehreren Abschnitten'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div id="intro-section">
    <h2>Introduction</h2>
    <p>Content for the introduction...</p>
</div>

<div id="main-section">
    <h2>Main Content</h2>
    <p>Content for the main article...</p>
</div>

<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    // Initialisiere den Intro-Abschnitt
    FastCommentsCollabChat(document.getElementById('intro-section'), {
        tenantId: 'demo',
        urlId: 'my-article-intro'
    });

    // Initialisiere den Hauptabschnitt
    FastCommentsCollabChat(document.getElementById('main-section'), {
        tenantId: 'demo',
        urlId: 'my-article-main'
    });
</script>
[inline-code-end]