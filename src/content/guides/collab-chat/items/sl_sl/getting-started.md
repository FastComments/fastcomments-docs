### Hitri začetek

Začetek uporabe Collab Chat je preprost. Potrebujete skripto FastComments Collab Chat, element HTML, ki vsebuje besedilo, ki ga želite označiti, in konfiguracijski objekt z vašim Tenant ID.

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

Zamenjajte `'demo'` z vašim dejanskim FastComments Tenant ID, če tega še niste storili, ki ga najdete v svoji [nadzorni plošči FastComments](https://fastcomments.com/auth/my-account/api-secret).

### Kako deluje

Ko je enkrat inicializiran, lahko uporabniki izberejo katero koli besedilo znotraj ciljnega elementa. Po kratkem zamiku (3,5 sekunde na namizju) se prikaže poziv, ki jim omogoča začetek razprave. Ko je razprava ustvarjena, se na besedilu prikaže vizualna oznaka. Drugi uporabniki lahko z miško zapeljejo čez oznako ali kliknejo nanjo, da si ogledajo razpravo in se ji pridružijo. Vse razprave se v realnem času sinhronizirajo med vsemi obiskovalci.

### Živi demo

Collab Chat v akciji si lahko ogledate na naši [strani z živim demom](https://fastcomments.com/product/collab-chat).

### Naslednji koraki

Zdaj, ko imate osnove v delovanju, lahko prilagodite videz in vedenje v vodiču Configuration Options. Oglejte si vodič Text Selection Behavior, da razumete, kako deluje izbira besedila. Naučite se o oblikovanju in podpori za temni način v vodiču Customization. Za naprednejše integracije raziščite API Reference.

### Frontend knjižnice

Vse FastComments frontend knjižnice (react, vue, angular itd.) imajo Collab Chat.