### Brzi početak

Početak rada sa Collab Chat je jednostavan. Potreban vam je FastComments Collab Chat skript, HTML element koji sadrži tekst koji želite da anotirate, i konfiguracioni objekat sa vašim Tenant ID.

### Instalacija

Dodajte Collab Chat skript na svoju stranicu:

[inline-code-attrs-start title = 'Učitavanje Collab Chat skripta'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### Osnovna implementacija

Evo minimalnog primera:

[inline-code-attrs-start title = 'Osnovna implementacija Collab Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <!-- Vaš kontejner sadržaja -->
    <div id="article-content" style="min-height: 500px;">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <!-- Učitavanje Collab Chat skripta -->
    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>

    <!-- Inicijalizacija Collab Chata -->
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Zamenite `'demo'` svojim stvarnim FastComments Tenant ID-jem ako već nije, koji možete pronaći u svom [FastComments kontrolnoj tabli](https://fastcomments.com/auth/my-account/api-secret).

### Kako funkcioniše

Kada je inicijalizovan, korisnici mogu selektovati bilo koji tekst unutar ciljanog elementa. Nakon kratkog kašnjenja (3.5 seconds on desktop), pojavljuje se prompt koji im omogućava da započnu diskusiju. Kada se diskusija kreira, pojavljuje se vizuelno isticanje na tekstu. Ostali korisnici mogu preći mišem preko ili kliknuti na isticanje da vide i učestvuju u diskusiji. Sve diskusije se sinhronizuju u realnom vremenu među svim posetiocima.

### Uživo demo

Možete videti Collab Chat u akciji na našoj [stranici demonstracije uživo](https://fastcomments.com/product/collab-chat).

### Sledeći koraci

Sada kada imate osnovno funkcionisanje, možete prilagoditi izgled i ponašanje u vodiču za opcije konfiguracije. Pogledajte vodič o ponašanju pri selekciji teksta da biste razumeli kako izbor teksta funkcioniše. Saznajte o stilizaciji i podršci za tamni režim u vodiču za prilagođavanje. Za napredne integracije, istražite Referencu API-ja.

### Frontend biblioteke

Sve FastComments frontend biblioteke (react, vue, angular, itd.) imaju Collab Chat.