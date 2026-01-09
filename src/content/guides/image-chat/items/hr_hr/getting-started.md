### Slučajevi upotrebe

Image Chat izvrsno funkcionira za povratne informacije o dizajnu gdje timovi trebaju raspravljati o određenim elementima u maketama ili snimkama zaslona. Web‑stranice za recenzije proizvoda mogu omogućiti korisnicima raspravu o specifičnim značajkama vidljivim na fotografijama proizvoda. Obrazovne platforme mogu ga koristiti za raspravu o dijagramima, kartama ili znanstvenim slikama. Foto galerije mogu postati interaktivne s komentarima vezanim uz određene lokacije. Stranice nekretnina mogu dopustiti gledateljima da raspravljaju o pojedinim značajkama vidljivim na fotografijama objekata.

### Brzi početak

Početak rada s Image Chatom je jednostavan. Trebate FastComments Image Chat skriptu, element slike ili spremnik sa slikom i konfiguracijski objekt s vašim Tenant ID.

### Instalacija

Dodajte Image Chat skriptu na svoju stranicu:

[inline-code-attrs-start title = 'Učitavanje Image Chat skripte'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
[inline-code-end]

### Osnovna implementacija

Evo minimalnog primjera:

[inline-code-attrs-start title = 'Osnovna implementacija Image Chata'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

    <!-- Inicijalizirajte Image Chat -->
    <script>
        FastCommentsImageChat(document.getElementById('my-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Zamijenite ` 'demo' ` sa svojim stvarnim FastComments Tenant ID-om ako već nije, koji možete pronaći u svojoj [FastComments dashboard](https://fastcomments.com/auth/my-account).

### Kako radi

Nakon inicijalizacije, korisnici mogu kliknuti bilo gdje na slici. Kada se dogodi klik, na toj lokaciji pojavi se vizualni kvadratni marker i otvori se chat prozor. Ostali korisnici mogu vidjeti sve markere i kliknuti ih da pregledaju ili sudjeluju u tim raspravama. Sve rasprave se sinkroniziraju u stvarnom vremenu među svim posjetiteljima.

Widget koristi pozicioniranje temeljeno na postotcima, pa markeri ostaju na ispravnom mjestu čak i kada se slika promijeni veličinu ili se pregledava na različitim veličinama zaslona.

### Demo uživo

Možete vidjeti Image Chat u akciji na našoj [demo stranici uživo](https://fastcomments.com/product/image-chat).

### Sljedeći koraci

Sada kada imate osnovne funkcionalnosti, možete prilagoditi izgled i ponašanje u vodiču Configuration Options. Pogledajte vodič Responsive Design kako biste razumjeli kako funkcionira pozicioniranje temeljeno na postotcima. Saznajte o stiliziranju i podršci za tamni način rada u vodiču Customization. Za napredne integracije istražite API Reference.

### Frontend biblioteke

Sve FastComments frontend biblioteke (react, vue, angular, itd.) imaju Image Chat.