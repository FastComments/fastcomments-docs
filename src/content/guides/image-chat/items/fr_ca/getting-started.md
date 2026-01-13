### Cas d'utilisation

Image Chat est idéal pour les retours de conception lorsque les équipes doivent discuter d'éléments spécifiques dans des maquettes ou des captures d'écran. Les sites d'avis sur les produits peuvent permettre aux clients de discuter de caractéristiques précises visibles sur des photos de produits. Les plateformes éducatives peuvent l'utiliser pour discuter de diagrammes, de cartes ou d'images scientifiques. Les galeries photo peuvent devenir interactives avec des commentaires localisés. Les sites immobiliers peuvent permettre aux visiteurs de discuter d'éléments visibles sur les photos des propriétés.

### Démarrage rapide

Commencer avec Image Chat est simple. Vous avez besoin du script FastComments Image Chat, d'un élément image ou d'un conteneur contenant une image, et d'un objet de configuration avec votre Tenant ID.

### Installation

Ajoutez le script Image Chat à votre page :

[inline-code-attrs-start title = 'Chargement du script Image Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
[inline-code-end]

### Implémentation de base

Voici un exemple minimal :

[inline-code-attrs-start title = 'Implémentation de base pour Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Image Gallery with Image Chat</title>
</head>
<body>
    <!-- Votre image -->
    <img id="my-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Pretty Trail" />

    <!-- Charger le script Image Chat -->
    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>

    <!-- Initialiser Image Chat -->
    <script>
        FastCommentsImageChat(document.getElementById('my-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Remplacez `'demo'` par votre véritable FastComments Tenant ID si ce n'est pas déjà fait, que vous pouvez trouver dans votre [tableau de bord FastComments](https://fastcomments.com/auth/my-account).

### Comment ça fonctionne

Une fois initialisé, les utilisateurs peuvent cliquer n'importe où sur l'image. Lorsqu'un clic se produit, un marqueur carré visuel apparaît à cet emplacement et une fenêtre de discussion s'ouvre. Les autres utilisateurs peuvent voir tous les marqueurs et cliquer dessus pour voir ou participer à ces discussions. Toutes les discussions se synchronisent en temps réel entre tous les visiteurs.

Le widget utilise un positionnement en pourcentage, donc les marqueurs restent à la bonne position même lorsque l'image redimensionne ou est affichée sur différentes tailles d'écran.

### Démo en direct

Vous pouvez voir Image Chat en action sur notre [page de démonstration en direct](https://fastcomments.com/product/image-chat).

### Prochaines étapes

Maintenant que vous avez les bases en place, vous pouvez personnaliser l'apparence et le comportement dans le guide des options de configuration. Consultez le guide de conception réactive pour comprendre comment fonctionne le positionnement en pourcentage. Informez-vous sur le style et la prise en charge du mode sombre dans le guide de personnalisation. Pour des intégrations avancées, explorez la Référence de l'API.

### Bibliothèques frontend

Toutes les bibliothèques frontend FastComments (react, vue, angular, etc) incluent Image Chat.