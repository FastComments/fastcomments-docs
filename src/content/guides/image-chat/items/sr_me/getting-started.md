### Use Cases

Image Chat odlično funkcioniše za povratne informacije o dizajnu kada timovi treba da diskutuju o određenim elementima u maketama ili snimcima ekrana. Sajtovi za recenzije proizvoda mogu omogućiti kupcima da raspravljaju o određenim karakteristikama vidljivim na fotografijama proizvoda. Obrazovne platforme mogu koristiti Image Chat za diskusiju o dijagramima, mapama ili naučnim slikama. Foto galerije mogu postati interaktivne sa komentarima vezanim za određenu lokaciju na slici. Sajtovi za nekretnine mogu omogućiti posjetiocima da diskutuju o određenim karakteristikama vidljivim na fotografijama nekretnina.

### Quick Start

Početak rada sa Image Chat-om je jednostavan. Potrebni su vam FastComments Image Chat skripta, element slike ili kontejner sa slikom, i konfiguracioni objekat sa vašim Tenant ID-jem.

### Installation

Add the Image Chat script to your page:

[inline-code-attrs-start title = 'Učitavanje Image Chat skripte'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
[inline-code-end]

### Basic Implementation

Here's a minimal example:

[inline-code-attrs-start title = 'Osnovna implementacija Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

Replace `'demo'` with your actual FastComments Tenant ID if it's not already, which you can find in your [FastComments dashboard](https://fastcomments.com/auth/my-account).

### How It Works

Once initialized, users can click anywhere on the image. When a click occurs, a visual square marker appears at that location and a chat window opens. Other users can see all the markers and click them to view or participate in those discussions. All discussions sync in real-time across all visitors.

The widget uses percentage-based positioning, so markers stay in the correct location even when the image resizes or is viewed on different screen sizes.

### Live Demo

You can see Image Chat in action on our [stranici s prikazom uživo](https://fastcomments.com/product/image-chat).

### Next Steps

Now that you have the basics working, you can customize the appearance and behavior in the Configuration Options guide. Check out the Responsive Design guide to understand how percentage-based positioning works. Learn about styling and dark mode support in the Customization guide. For advanced integrations, explore the API Reference.

### Frontend Libraries

All FastComments frontend libraries (react, vue, angular, etc) have Image Chat.