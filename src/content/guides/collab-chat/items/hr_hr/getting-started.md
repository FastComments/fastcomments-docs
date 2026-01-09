### Brzi početak

Početak rada s Collab Chatom je jednostavan. Potrebni su vam FastComments Collab Chat skripta, HTML element koji sadrži tekst koji želite anotirati te konfiguracijski objekt s vašim Tenant ID-om.

### Instalacija

Dodajte Collab Chat skriptu na svoju stranicu:

[inline-code-attrs-start title = 'Učitavanje Collab Chat skripte'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### Osnovna implementacija

Evo minimalnog primjera:

[inline-code-attrs-start title = 'Osnovna implementacija Collab Chata'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <!-- Vaš spremnik sadržaja -->
    <div id="article-content" style="min-height: 500px;">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <!-- Učitaj Collab Chat skriptu -->
    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>

    <!-- Inicijaliziraj Collab Chat -->
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Zamijenite `'demo'` sa svojim stvarnim FastComments Tenant ID-om ako već nije, koji možete pronaći u svojem [FastComments nadzornom panelu](https://fastcomments.com/auth/my-account/api-secret).

### Kako to radi

Kada je inicijalizirano, korisnici mogu odabrati bilo koji tekst unutar ciljanog elementa. Nakon kratke odgode (3.5 sekundi na desktopu), pojavljuje se upit koji im omogućuje pokretanje rasprave. Kada je rasprava stvorena, na tekstu se pojavljuje vizualno isticanje. Drugi korisnici mogu prijeći mišem ili kliknuti na istaknuti tekst da bi pregledali i sudjelovali u raspravi. Sve rasprave se sinkroniziraju u stvarnom vremenu među svim posjetiteljima.

### Demo uživo

Collab Chat možete vidjeti u akciji na našoj [stranici s demo prikazom uživo](https://fastcomments.com/product/collab-chat).

### Sljedeći koraci

Sada kada imate osnovno funkcionalnost, možete prilagoditi izgled i ponašanje u vodiču za Konfiguracijske opcije. Pogledajte vodič o ponašanju odabira teksta kako biste razumjeli kako funkcionira odabir teksta. Saznajte o stiliziranju i podršci za tamni način rada u vodiču za Prilagodbu. Za napredne integracije, istražite API Reference.

### Frontend biblioteke

Sve FastComments frontend biblioteke (react, vue, angular, itd.) imaju Collab Chat.

---