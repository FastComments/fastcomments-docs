### Cas d'utilisation

Image Chat fonctionne parfaitement pour les retours sur des maquettes, lorsque les équipes doivent discuter d'éléments spécifiques dans des mockups ou des captures d'écran. Les sites d'avis produits peuvent permettre aux clients de discuter de fonctionnalités spécifiques visibles sur les photos des produits. Les plateformes éducatives peuvent l'utiliser pour discuter de diagrammes, de cartes ou d'images scientifiques. Les galeries photo peuvent devenir interactives avec des commentaires localisés. Les sites immobiliers peuvent permettre aux visiteurs de discuter de caractéristiques spécifiques visibles sur les photos des biens.

### Démarrage rapide

Commencer avec Image Chat est simple. Vous avez besoin du script FastComments Image Chat, d'un élément image ou d'un conteneur avec une image, et d'un objet de configuration avec votre Tenant ID.

### Installation

Ajoutez le script Image Chat à votre page :

[inline-code-attrs-start title = 'Chargement du script Image Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
[inline-code-end]

### Implémentation de base

Voici un exemple minimal :

[inline-code-attrs-start title = 'Implémentation de base d\'Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Image Gallery with Image Chat</title>
</head>
<body>
    <!-- Your image -->
    <img id="my-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Pretty Trail" />

    <!-- Load the Image Chat script -->
    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>

    <!-- Initialize Image Chat -->
    <script>
        FastCommentsImageChat(document.getElementById('my-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Remplacez `'demo'` par votre véritable Tenant ID FastComments si ce n'est pas déjà fait, que vous pouvez trouver dans votre [tableau de bord FastComments](https://fastcomments.com/auth/my-account).

### Fonctionnement

Une fois initialisé, les utilisateurs peuvent cliquer n'importe où sur l'image. Lorsqu'un clic se produit, un marqueur visuel carré apparaît à cet emplacement et une fenêtre de discussion s'ouvre. Les autres utilisateurs peuvent voir tous les marqueurs et cliquer dessus pour consulter ou participer à ces discussions. Toutes les discussions se synchronisent en temps réel pour tous les visiteurs.

Le widget utilise un positionnement en pourcentage, de sorte que les marqueurs restent au bon endroit même lorsque l'image est redimensionnée ou consultée sur différentes tailles d'écran.

### Démo en direct

Vous pouvez voir Image Chat en action sur notre [page de démo en direct](https://fastcomments.com/product/image-chat).

### Prochaines étapes

Maintenant que vous avez les bases en place, vous pouvez personnaliser l'apparence et le comportement dans le guide des Options de configuration. Consultez le guide du Design responsive pour comprendre comment fonctionne le positionnement en pourcentage. Découvrez le style et la prise en charge du mode sombre dans le guide de Personnalisation. Pour des intégrations avancées, explorez la Référence API.

### Bibliothèques frontend

Toutes les bibliothèques frontend FastComments (react, vue, angular, etc.) incluent Image Chat.