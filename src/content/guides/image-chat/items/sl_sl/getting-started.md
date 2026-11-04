### Primeri uporabe

Image Chat odlično deluje za povratne informacije o oblikovanju, kjer morajo ekipe razpravljati o določenih elementih v maketah ali posnetkih zaslona. Spletna mesta za ocenjevanje izdelkov lahko strankam omogočijo razpravo o določenih funkcijah, vidnih na fotografijah izdelkov. Izobraževalne platforme ga lahko uporabijo za razpravo o diagramih, zemljevidih ali znanstvenih slikah. Fotogalerije lahko postanejo interaktivne s komentarji, vezanimi na lokacijo. Spletna mesta za nepremičnine lahko obiskovalcem omogočijo razpravo o določenih lastnostih, vidnih na fotografijah nepremičnin.

### Hitri začetek

Začetek uporabe Image Chat je preprost. Potrebujete skripto FastComments Image Chat, element ali vsebnik s sliko ter konfiguracijski objekt z vašim Tenant ID.

### Namestitev

Dodajte skripto Image Chat na svojo stran:

[inline-code-attrs-start title = 'Nalaganje skripte Image Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
[inline-code-end]

### Osnovna implementacija

Tukaj je minimalen primer:

[inline-code-attrs-start title = 'Osnovna implementacija Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>Moja fotogalerija z Image Chat</title>
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

Zamenjajte `'demo'` s svojim dejanskim FastComments Tenant ID, če tega še niste storili. Najdete ga v svoji [nadzorni plošči FastComments](https://fastcomments.com/auth/my-account).

### Kako deluje

Ko je enkrat inicializiran, lahko uporabniki kliknejo kjerkoli na sliki. Ko pride do klika, se na tej lokaciji prikaže vizualni kvadratni označevalnik in odpre se okno za klepet. Drugi uporabniki lahko vidijo vse označevalnike in kliknejo nanje, da si ogledajo ali sodelujejo v teh razpravah. Vse razprave se v realnem času sinhronizirajo med vsemi obiskovalci.

Gradnik uporablja pozicioniranje na osnovi odstotkov, tako da označevalniki ostanejo na pravilni lokaciji tudi, ko se slika spremeni velikost ali si jo ogledujejo na različnih velikostih zaslonov.

### Predstavitev v živo

Image Chat v akciji si lahko ogledate na naši [strani predstavitve v živo](https://fastcomments.com/product/image-chat).

### Naslednji koraki

Zdaj, ko imate osnove v delovanju, lahko prilagodite videz in vedenje v vodiču Configuration Options. Oglejte si vodič Responsive Design, da razumete, kako deluje pozicioniranje na osnovi odstotkov. Naučite se o oblikovanju in podpori za temni način v vodiču Customization. Za napredne integracije raziščite API Reference.

### Frontend knjižnice

Vse FastComments frontend knjižnice (react, vue, angular itd.) vključujejo Image Chat.