### Use Cases

Image Chat je odličan za povratne informacije o dizajnu kada timovi trebaju diskutovati o specifičnim elementima u maketama ili snimkama ekrana. Sajtovi za recenzije proizvoda mogu omogućiti korisnicima da raspravljaju o specifičnim karakteristikama vidljivim na fotografijama proizvoda. Obrazovne platforme mogu ga koristiti za raspravu o dijagramima, mapama ili naučnim slikama. Foto galerije mogu postati interaktivne uz komentare vezane za određene lokacije na slici. Nekretninski sajtovi mogu omogućiti posjetiocima da raspravljaju o specifičnim značajkama vidljivim na fotografijama nekretnina.

### Quick Start

Početi sa Image Chat-om je jednostavno. Potreban vam je FastComments Image Chat script, element slike ili kontejner sa slikom, i konfiguracijski objekt sa vašim Tenant ID.

### Installation

Add the Image Chat script to your page:

[inline-code-attrs-start title = 'Učitavanje Image Chat skripte'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
[inline-code-end]

### Basic Implementation

Here's a minimal example:

[inline-code-attrs-start title = 'Osnovna implementacija Image Chat-a'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Image Gallery with Image Chat</title>
</head>
<body>
    <!-- Vaša slika -->
    <img id="my-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Pretty Trail" />

    <!-- Učitajte Image Chat skriptu -->
    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>

    <!-- Inicijalizujte Image Chat -->
    <script>
        FastCommentsImageChat(document.getElementById('my-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Replace `'demo'` with your actual FastComments Tenant ID if it's not already, which you can find in your [FastComments kontrolna ploča](https://fastcomments.com/auth/my-account).

### How It Works

Once initialized, users can click anywhere on the image. When a click occurs, a visual square marker appears at that location and a chat window opens. Other users can see all the markers and click them to view or participate in those discussions. All discussions sync in real-time across all visitors.

The widget uses percentage-based positioning, so markers stay in the correct location even when the image resizes or is viewed on different screen sizes.

### Live Demo

You can see Image Chat in action on our [stranici s demo verzijom uživo](https://fastcomments.com/product/image-chat).

### Next Steps

Now that you have the basics working, you can customize the appearance and behavior in the Configuration Options guide. Check out the Responsive Design guide to understand how percentage-based positioning works. Learn about styling and dark mode support in the Customization guide. For advanced integrations, explore the API Reference.

### Frontend Libraries

All FastComments frontend libraries (react, vue, angular, etc) have Image Chat.