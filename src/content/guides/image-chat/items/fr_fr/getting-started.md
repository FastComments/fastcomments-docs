### Use Cases

Image Chat fonctionne très bien pour les retours de conception où les équipes doivent discuter d'éléments spécifiques dans des maquettes ou des captures d'écran. Les sites d'avis produits peuvent permettre aux clients de discuter des fonctionnalités visibles sur les photos des produits. Les plateformes éducatives peuvent l'utiliser pour discuter de diagrammes, de cartes ou d'images scientifiques. Les galeries photo peuvent devenir interactives avec des commentaires localisés. Les sites immobiliers peuvent permettre aux visiteurs de discuter des caractéristiques visibles sur les photos des biens.

### Quick Start

Commencer avec Image Chat est simple. Vous avez besoin du script FastComments Image Chat, d'un élément d'image ou d'un conteneur avec une image, et d'un objet de configuration contenant votre Tenant ID.

### Installation

Add the Image Chat script to your page:

[inline-code-attrs-start title = 'Chargement du script Image Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
[inline-code-end]

### Basic Implementation

Here's a minimal example:

[inline-code-attrs-start title = 'Implémentation basique pour Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

Replace `'demo'` with your actual FastComments Tenant ID if it's not already, which you can find in your [tableau de bord FastComments](https://fastcomments.com/auth/my-account).

### How It Works

Once initialized, users can click anywhere on the image. When a click occurs, a visual square marker appears at that location and a chat window opens. Other users can see all the markers and click them to view or participate in those discussions. All discussions sync in real-time across all visitors.

The widget uses percentage-based positioning, so markers stay in the correct location even when the image resizes or is viewed on different screen sizes.

### Live Demo

You can see Image Chat in action on our [page de démonstration en direct](https://fastcomments.com/product/image-chat).

### Next Steps

Now that you have the basics working, you can customize the appearance and behavior in the Configuration Options guide. Check out the Responsive Design guide to understand how percentage-based positioning works. Learn about styling and dark mode support in the Customization guide. For advanced integrations, explore the API Reference.

### Frontend Libraries

All FastComments frontend libraries (react, vue, angular, etc) have Image Chat.