### Slučajevi upotrebe

Image Chat je odličan za povratne informacije o dizajnu kada timovi treba da diskutuju o specifičnim elementima na maketama ili snimcima ekrana. Sajtovi za recenzije proizvoda mogu dozvoliti kupcima da diskutuju o konkretnim karakteristikama vidljivim na fotografijama proizvoda. Obrazovne platforme mogu ga koristiti za diskusiju o dijagramima, mapama ili naučnim slikama. Foto galerije mogu postati interaktivne sa komentarima vezanim za određene lokacije. Sajtovi za nekretnine mogu omogućiti posetiocima da diskutuju o specifičnim detaljima vidljivim na fotografijama nekretnina.

### Brzi početak

Početak rada sa Image Chat-om je jednostavan. Potreban vam je FastComments Image Chat skript, element slike ili kontejner sa slikom, i konfiguracioni objekat sa vašim Tenant ID.

### Instalacija

Dodajte Image Chat skriptu na vašu stranicu:

[inline-code-attrs-start title = 'Učitavanje Image Chat skripte'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
[inline-code-end]

### Osnovna implementacija

Evo minimalnog primera:

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

    <!-- Učitaj Image Chat skriptu -->
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

Zamenite `'demo'` sa vašim stvarnim FastComments Tenant ID-jem ako već nije, koji možete pronaći na vašoj [FastComments kontrolnoj tabli](https://fastcomments.com/auth/my-account).

### Kako funkcioniše

Kada je inicijalizovan, korisnici mogu kliknuti bilo gde na slici. Kada se klik dogodi, na toj lokaciji se pojavi vizuelni kvadratni marker i otvori se chat prozor. Drugi korisnici mogu videti sve markere i kliknuti na njih da pregledaju ili učestvuju u tim diskusijama. Sve diskusije se sinhronizuju u realnom vremenu među svim posetiocima.

Widžet koristi pozicioniranje zasnovano na procentima, tako da markeri ostaju na tačnoj lokaciji čak i kada se slika promeni veličinu ili se pregleda na različitim veličinama ekrana.

### Demo uživo

Možete videti Image Chat u akciji na našoj [live demo stranici](https://fastcomments.com/product/image-chat).

### Sledeći koraci

Sada kada imate osnovno rešenje, možete prilagoditi izgled i ponašanje u vodiču za opcije konfiguracije. Pogledajte vodič za responzivni dizajn da biste razumeli kako funkcioniše pozicioniranje zasnovano na procentima. Saznajte više o stilizaciji i podršci za tamni režim u vodiču za prilagođavanje. Za napredne integracije, istražite API referencu.

### Frontend biblioteke

Sve FastComments frontend biblioteke (react, vue, angular, itd.) uključuju Image Chat.

---