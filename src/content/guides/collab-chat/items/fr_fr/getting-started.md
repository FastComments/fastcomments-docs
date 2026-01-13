### Quick Start

Commencer avec Collab Chat est simple. Vous avez besoin du script FastComments Collab Chat, d'un élément HTML contenant le texte que vous souhaitez annoter, et d'un objet de configuration avec votre Tenant ID.

### Installation

Ajoutez le script Collab Chat à votre page :

[inline-code-attrs-start title = 'Chargement du script Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### Basic Implementation

Voici un exemple minimal :

[inline-code-attrs-start title = 'Implémentation de base de Collab Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <!-- Votre conteneur de contenu -->
    <div id="article-content" style="min-height: 500px;">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <!-- Charger le script Collab Chat -->
    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>

    <!-- Initialiser Collab Chat -->
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Remplacez `'demo'` par votre véritable Tenant ID FastComments si ce n'est pas déjà fait, que vous pouvez trouver dans votre [tableau de bord FastComments](https://fastcomments.com/auth/my-account/api-secret).

### How It Works

Une fois initialisé, les utilisateurs peuvent sélectionner n'importe quel texte dans l'élément ciblé. Après un bref délai (3,5 secondes sur desktop), une invite apparaît leur permettant de démarrer une discussion. Lorsqu'une discussion est créée, une mise en évidence visuelle apparaît sur le texte. D'autres utilisateurs peuvent survoler ou cliquer sur la mise en évidence pour voir et participer à la discussion. Toutes les discussions se synchronisent en temps réel entre tous les visiteurs.

### Live Demo

Vous pouvez voir Collab Chat en action sur notre [page de démo en direct](https://fastcomments.com/product/collab-chat).

### Next Steps

Maintenant que vous avez les bases en place, vous pouvez personnaliser l'apparence et le comportement dans le guide Configuration Options. Consultez le guide Text Selection Behavior pour comprendre comment fonctionne la sélection de texte. Informez-vous sur le style et la prise en charge du mode sombre dans le guide Customization. Pour des intégrations avancées, explorez la API Reference.

### Frontend Libraries

Toutes les bibliothèques frontend FastComments (react, vue, angular, etc.) incluent Collab Chat.

---