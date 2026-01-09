### Use Cases

Image Chat works great for design feedback where teams need to discuss specific elements in mockups or screenshots. Product review sites can let customers discuss specific features visible in product photos. Educational platforms can use it for discussing diagrams, maps, or scientific images. Photo galleries can become interactive with location-specific commentary. Real estate sites can let viewers discuss specific features visible in property photos.

### Quick Start

Getting started with Image Chat is simple. You need the FastComments Image Chat script, an image element or container with an image, and a configuration object with your Tenant ID.

### Installation

Add the Image Chat script to your page:

[inline-code-attrs-start title = 'Carregando o Script do Image Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
[inline-code-end]

### Basic Implementation

Here's a minimal example:

[inline-code-attrs-start title = 'Implementação Básica do Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Image Gallery with Image Chat</title>
</head>
<body>
    <!-- Sua imagem -->
    <img id="my-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Pretty Trail" />

    <!-- Carregue o script do Image Chat -->
    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>

    <!-- Inicialize o Image Chat -->
    <script>
        FastCommentsImageChat(document.getElementById('my-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Replace `'demo'` with your actual FastComments Tenant ID if it's not already, which you can find in your [FastComments dashboard](https://fastcomments.com/auth/my-account).

### How It Works

Once initialized, users can click anywhere on the image. When a click occurs, a visual square marker appears at that location and a chat window opens. Other users can see all the markers and click them to view or participate in those discussions. All discussions sync in real-time across all visitors.

The widget uses percentage-based positioning, so markers stay in the correct location even when the image resizes or is viewed on different screen sizes.

### Live Demo

You can see Image Chat in action on our [live demo page](https://fastcomments.com/product/image-chat).

### Next Steps

Now that you have the basics working, you can customize the appearance and behavior in the Configuration Options guide. Check out the Responsive Design guide to understand how percentage-based positioning works. Learn about styling and dark mode support in the Customization guide. For advanced integrations, explore the API Reference.

### Frontend Libraries

All FastComments frontend libraries (react, vue, angular, etc) have Image Chat.