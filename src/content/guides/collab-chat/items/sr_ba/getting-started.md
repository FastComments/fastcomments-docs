### Brzi početak

Početak rada sa Collab Chat je jednostavan. Potreban vam je FastComments Collab Chat skripta, HTML element koji sadrži tekst koji želite komentarisati, i konfiguracioni objekat sa vašim Tenant ID.

### Instalacija

Dodajte Collab Chat skriptu na vašu stranicu:

[inline-code-attrs-start title = 'Učitavanje Collab Chat skripte'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### Osnovna implementacija

Evo minimalnog primera:

[inline-code-attrs-start title = 'Osnovna implementacija Collab Chata'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <!-- Your content container -->
    <div id="article-content" style="min-height: 500px;">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <!-- Load the Collab Chat script -->
    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>

    <!-- Initialize Collab Chat -->
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Zamijenite `'demo'` sa vašim stvarnim FastComments Tenant ID ako već nije, koji možete pronaći u vašem [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

### Kako funkcioniše

Kada se inicijalizuje, korisnici mogu izabrati bilo koji tekst unutar ciljanog elementa. Nakon kratkog kašnjenja (3.5 sekunde na desktopu), pojavljuje se prompt koji im omogućava da započnu diskusiju. Kada se kreira diskusija, na tekstu se pojavi vizuelno isticanje. Drugi korisnici mogu preći mišem ili kliknuti na istaknuti dio da vide i učestvuju u diskusiji. Sve diskusije se sinhronizuju u realnom vremenu za sve posjetioce.

### Demo uživo

Možete vidjeti Collab Chat u akciji na našoj [stranici demonstracije uživo](https://fastcomments.com/product/collab-chat).

### Sledeći koraci

Sada kada imate osnovno funkcionisanje, možete prilagoditi izgled i ponašanje u vodiču za Configuration Options. Pogledajte vodič o ponašanju pri odabiru teksta da biste razumjeli kako selekcija teksta funkcioniše. Saznajte o stilizaciji i podršci za tamni režim u vodiču za Customization. Za napredne integracije, istražite API referencu.

### Frontend biblioteke

Sve FastComments frontend biblioteke (react, vue, angular, etc) imaju Collab Chat.

---