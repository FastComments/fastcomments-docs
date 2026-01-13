### Primeri uporabe

Image Chat je odličen za povratne informacije o oblikovanju, kjer ekipe potrebujejo razpravo o določenih elementih v maketah ali posnetkih zaslona. Spletne strani z ocenami izdelkov lahko omogočajo strankam razpravo o določenih značilnostih, vidnih na fotografijah izdelkov. Izobraževalne platforme ga lahko uporabijo za razpravo o diagramih, zemljevidih ali znanstvenih slikah. Foto galerije lahko postanejo interaktivne z lokacijsko specifičnimi komentarji. Nepremičninske strani lahko obiskovalcem omogočijo razpravo o določenih značilnostih, vidnih na fotografijah nepremičnin.

### Hiter začetek

Zagon Image Chat je preprost. Potrebujete FastComments Image Chat skripto, element slike ali vsebnik s sliko ter konfiguracijski objekt z vašo Tenant ID.

### Namestitev

Add the Image Chat script to your page:

[inline-code-attrs-start title = 'Nalaganje skripte Image Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
[inline-code-end]

### Osnovna implementacija

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

Zamenjajte `'demo'` z vašim dejanskim FastComments Tenant ID, če ni že nastavljen; najdete ga v svoji [nadzorni plošči FastComments](https://fastcomments.com/auth/my-account).

### Kako deluje

Ko je inicializiran, lahko uporabniki kliknejo kjerkoli na sliko. Ko klik nastane, se na tem mestu prikaže vizualni kvadratni marker in odpre se klepetalno okno. Drugi uporabniki lahko vidijo vse markerje in nanje kliknejo, da si ogledajo ali sodelujejo v teh razpravah. Vse razprave se sinhronizirajo v realnem času med vsemi obiskovalci.

Pripomoček uporablja pozicioniranje na osnovi odstotkov, zato markerji ostanejo na pravilnem mestu tudi, ko se slika spremeni velikost ali je ogledana na različnih velikostih zaslonov.

### Živa predstavitev

Image Chat si lahko ogledate v akciji na naši [strani žive predstavitve](https://fastcomments.com/product/image-chat).

### Naslednji koraki

Zdaj, ko imate osnovno delujoče, lahko prilagodite videz in vedenje v vodniku Možnosti konfiguracije. Oglejte si vodnik o odzivnem oblikovanju, da razumete, kako deluje pozicioniranje na osnovi odstotkov. Naučite se o stilizaciji in podpori temnega načina v vodniku za prilagajanje. Za napredne integracije raziščite API referenco.

### Frontend knjižnice

Vse FastComments frontend knjižnice (react, vue, angular itd.) vključujejo Image Chat.

---