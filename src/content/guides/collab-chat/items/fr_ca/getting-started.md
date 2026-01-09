### Démarrage rapide

Commencer avec Collab Chat est simple. Vous avez besoin du script FastComments Collab Chat, d'un élément HTML contenant le texte que vous voulez annoter, et d'un objet de configuration avec votre Tenant ID.

### Installation

Ajoutez le script Collab Chat à votre page :

[inline-code-attrs-start title = 'Chargement du script Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### Implémentation de base

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

Remplacez `'demo'` par votre FastComments Tenant ID réel si ce n'est pas déjà fait, que vous pouvez trouver dans votre [tableau de bord FastComments](https://fastcomments.com/auth/my-account/api-secret).

### Comment ça fonctionne

Une fois initialisé, les utilisateurs peuvent sélectionner n'importe quel texte à l'intérieur de l'élément cible. Après un bref délai (3,5 secondes sur ordinateur), une invite apparaît leur permettant de démarrer une discussion. Lorsqu'une discussion est créée, une surbrillance visuelle apparaît sur le texte. D'autres utilisateurs peuvent survoler ou cliquer la surbrillance pour voir et participer à la discussion. Toutes les discussions se synchronisent en temps réel entre tous les visiteurs.

### Démo en direct

Vous pouvez voir Collab Chat en action sur notre [page de démonstration en direct](https://fastcomments.com/product/collab-chat).

### Prochaines étapes

Maintenant que vous avez les bases en place, vous pouvez personnaliser l'apparence et le comportement dans le guide Configuration Options. Consultez le guide Text Selection Behavior pour comprendre le fonctionnement de la sélection de texte. Découvrez le support du style et du mode sombre dans le guide Customization. Pour des intégrations avancées, explorez la Référence de l'API.

### Bibliothèques frontend

Toutes les bibliothèques frontend FastComments (react, vue, angular, etc.) incluent Collab Chat.