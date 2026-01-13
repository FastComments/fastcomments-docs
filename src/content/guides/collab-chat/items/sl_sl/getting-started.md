### Hiter začetek

Začetek s Collab Chat je preprost. Potrebujete skripto FastComments Collab Chat, HTML element, ki vsebuje besedilo, ki ga želite označiti, in konfiguracijski objekt z vašim Tenant ID.

### Namestitev

Dodajte skripto Collab Chat na svojo stran:

[inline-code-attrs-start title = 'Nalaganje skripte Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### Osnovna implementacija

Tukaj je minimalen primer:

[inline-code-attrs-start title = 'Osnovna implementacija Collab Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <!-- Kotejner z vašo vsebino -->
    <div id="article-content" style="min-height: 500px;">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <!-- Naložite skripto Collab Chat -->
    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>

    <!-- Inicializirajte Collab Chat -->
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Replace `'demo'` with your actual FastComments Tenant ID if it's not already, which you can find in your [nadzorni plošči FastComments](https://fastcomments.com/auth/my-account/api-secret).

### Kako deluje

Ko je inicializirano, lahko uporabniki izberejo katerokoli besedilo znotraj ciljnega elementa. Po kratki zamudi (3,5 sekunde na namizju) se prikaže poziv, ki jim omogoči začetek razprave. Ko je razprava ustvarjena, se na besedilu prikaže vizualna označba. Drugi uporabniki se lahko z miško premaknejo nad ali kliknejo označbo, da si ogledajo in sodelujejo v razpravi. Vse razprave se sinhronizirajo v realnem času med vsemi obiskovalci.

### Predstavitev v živo

Collab Chat si lahko ogledate v akciji na naši [predstavitveni strani v živo](https://fastcomments.com/product/collab-chat).

### Naslednji koraki

Sedaj, ko imate osnovno delujoče, lahko prilagodite videz in vedenje v vodiču o možnostih konfiguracije. Oglejte si vodič o vedenju izbire besedila, da razumete, kako deluje izbira besedila. Več o stiliranju in podpori temnega načina preberite v vodiču za prilagajanje. Za napredne integracije raziščite referenco API-ja.

### Front-end knjižnice

Vse FastComments front-end knjižnice (react, vue, angular itd.) vključujejo Collab Chat.