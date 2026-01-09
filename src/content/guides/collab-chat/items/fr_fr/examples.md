### Exemple de base

La façon la plus simple d'utiliser Collab Chat est de cibler un seul conteneur de contenu. Cet exemple montre comment activer les annotations de texte sur un article :

[inline-code-attrs-start title = 'Exemple de base de Collab Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Exemple avec un identifiant d'URL personnalisé (par page de livre, article, etc.)

Par défaut, Collab Chat utilise l'URL de la page combinée au chemin de l'élément et à la plage de texte pour identifier les conversations. Vous pouvez fournir un `urlId` personnalisé pour mieux contrôler la façon dont les conversations sont regroupées :

[inline-code-attrs-start title = 'Collab Chat avec identifiant URL personnalisé'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    FastCommentsCollabChat(document.getElementById('article-content'), {
        tenantId: 'demo',
        urlId: 'book-one-page-2'
    });
</script>
[inline-code-end]

Ceci est utile si la structure de vos URL change mais que vous souhaitez conserver les mêmes conversations, ou si vous voulez partager les mêmes annotations de conversation sur plusieurs pages.

### Exemple avec le mode sombre

Si votre site a un fond sombre, activez la prise en charge du mode sombre pour vous assurer que l'interface de chat s'affiche correctement :

[inline-code-attrs-start title = 'Collab Chat avec le mode sombre'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Exemple avec la barre supérieure désactivée

Par défaut, Collab Chat affiche une barre supérieure avec le nombre d'utilisateurs et le nombre de discussions. Vous pouvez la désactiver :

[inline-code-attrs-start title = 'Collab Chat avec la barre supérieure désactivée'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Exemple avec callback de mise à jour du nombre de commentaires

Vous pouvez suivre quand des commentaires sont ajoutés ou mis à jour en utilisant le callback `commentCountUpdated` :

[inline-code-attrs-start title = 'Collab Chat avec callback de mise à jour du nombre de commentaires'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

### Exemple avec plusieurs sections

Vous pouvez initialiser Collab Chat sur plusieurs sections distinctes de votre page. Chaque section aura ses propres annotations indépendantes :

[inline-code-attrs-start title = 'Collab Chat sur plusieurs sections'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    // Initialize on intro section
    FastCommentsCollabChat(document.getElementById('intro-section'), {
        tenantId: 'demo',
        urlId: 'my-article-intro'
    });

    // Initialize on main section
    FastCommentsCollabChat(document.getElementById('main-section'), {
        tenantId: 'demo',
        urlId: 'my-article-main'
    });
</script>
[inline-code-end]

---