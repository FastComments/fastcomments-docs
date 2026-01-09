### Brzi početak

Početak rada sa Collab Chat je jednostavan. Potrebni su vam FastComments Collab Chat skripta, HTML element koji sadrži tekst koji želite da anotirate i objekat konfiguracije sa vašim Tenant ID.

### Instalacija

Dodajte Collab Chat skriptu na vašu stranicu:

[inline-code-attrs-start title = 'Učitavanje Collab Chat skripte'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    <!-- Vaš kontejner sa sadržajem -->
    <div id="article-content" style="min-height: 500px;">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <!-- Učitaj Collab Chat skriptu -->
    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>

    <!-- Inicijalizuj Collab Chat -->
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Zamenite `'demo'` sa vašim stvarnim FastComments Tenant ID-jem ako već nije, koji možete pronaći na vašem [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

### Kako radi

Kada je inicijalizovano, korisnici mogu selektovati bilo koji tekst unutar ciljnog elementa. Nakon kratkog zadržavanja (3.5 sekundi na desktopu), pojavljuje se upit koji im omogućava da započnu diskusiju. Kada se diskusija kreira, vizuelno isticanje se pojavljuje na tekstu. Drugi korisnici mogu preći mišem preko ili kliknuti na isticanje da bi videli i učestvovali u diskusiji. Sve diskusije se sinhronizuju u realnom vremenu među svim posetiocima.

### Demo uživo

Možete videti Collab Chat u akciji na našoj [stranici demonstracije uživo](https://fastcomments.com/product/collab-chat).

### Sledeći koraci

Sada kada imate osnovno funkcionalnost, možete prilagoditi izgled i ponašanje u Vodiču za opcije konfiguracije. Pogledajte Vodič za ponašanje pri izboru teksta da biste razumeli kako funkcioniše selekcija teksta. Saznajte o stilizaciji i podršci za tamni režim u Vodiču za prilagođavanje. Za napredne integracije, istražite API referencu.

### Frontend biblioteke

Sve FastComments frontend biblioteke (react, vue, angular, itd.) imaju Collab Chat.