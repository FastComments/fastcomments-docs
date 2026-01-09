### Avvio rapido

Iniziare con Collab Chat è semplice. Ti serve lo script FastComments Collab Chat, un elemento HTML contenente il testo che vuoi annotare e un oggetto di configurazione con il tuo Tenant ID.

### Installazione

Aggiungi lo script Collab Chat alla tua pagina:

[inline-code-attrs-start title = 'Caricamento dello script Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### Implementazione di base

Ecco un esempio minimo:

[inline-code-attrs-start title = 'Implementazione di base di Collab Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <!-- Contenitore del contenuto -->
    <div id="article-content" style="min-height: 500px;">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <!-- Carica lo script Collab Chat -->
    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>

    <!-- Inizializza Collab Chat -->
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Sostituisci `'demo'` con il tuo effettivo Tenant ID FastComments se non lo hai già, che puoi trovare nel tuo [dashboard di FastComments](https://fastcomments.com/auth/my-account/api-secret).

### Come funziona

Una volta inizializzato, gli utenti possono selezionare qualsiasi testo all'interno dell'elemento target. Dopo un breve ritardo (3,5 secondi su desktop), appare un prompt che permette di avviare una discussione. Quando viene creata una discussione, appare un'evidenziazione visiva sul testo. Altri utenti possono posizionare il cursore sopra o cliccare sull'evidenziazione per visualizzare e partecipare alla discussione. Tutte le discussioni si sincronizzano in tempo reale tra tutti i visitatori.

### Demo dal vivo

Puoi vedere Collab Chat in azione nella nostra [pagina demo dal vivo](https://fastcomments.com/product/collab-chat).

### Prossimi passi

Ora che hai le basi funzionanti, puoi personalizzare l'aspetto e il comportamento nella guida alle opzioni di configurazione. Consulta la guida sul comportamento della selezione del testo per capire come funziona la selezione del testo. Scopri gli stili e il supporto per la modalità scura nella guida alla personalizzazione. Per integrazioni avanzate, esplora il Riferimento API.

### Librerie frontend

Tutte le librerie frontend di FastComments (react, vue, angular, ecc.) includono Collab Chat.