### Casi d'uso

Image Chat funziona molto bene per il feedback sul design, dove i team devono discutere elementi specifici in mockup o screenshot. I siti di recensioni di prodotti possono consentire ai clienti di discutere funzionalità specifiche visibili nelle foto dei prodotti. Le piattaforme educative possono usarlo per discutere diagrammi, mappe o immagini scientifiche. Le gallerie fotografiche possono diventare interattive con commenti legati a punti specifici. I siti immobiliari possono consentire ai visitatori di discutere caratteristiche specifiche visibili nelle foto degli immobili.

### Avvio rapido

Iniziare con Image Chat è semplice. Ti serve lo script FastComments Image Chat, un elemento immagine o un contenitore con un'immagine, e un oggetto di configurazione con il tuo Tenant ID.

### Installazione

Aggiungi lo script Image Chat alla tua pagina:

[inline-code-attrs-start title = 'Caricamento dello script Image Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
[inline-code-end]

### Implementazione di base

Ecco un esempio minimale:

[inline-code-attrs-start title = 'Implementazione di base di Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

Sostituisci `'demo'` con il tuo effettivo Tenant ID FastComments se non lo è già, che puoi trovare nella tua [dashboard FastComments](https://fastcomments.com/auth/my-account).

### Come funziona

Una volta inizializzato, gli utenti possono fare clic in qualsiasi punto dell'immagine. Quando si verifica un clic, un marcatore quadrato visibile appare in quella posizione e si apre una finestra di chat. Gli altri utenti possono vedere tutti i marcatori e fare clic su di essi per visualizzare o partecipare a quelle discussioni. Tutte le discussioni si sincronizzano in tempo reale per tutti i visitatori.

Il widget utilizza un posizionamento basato su percentuali, quindi i marcatori restano nella posizione corretta anche quando l'immagine viene ridimensionata o visualizzata su schermi di dimensioni diverse.

### Demo dal vivo

Puoi vedere Image Chat in azione sulla nostra [pagina demo dal vivo](https://fastcomments.com/product/image-chat).

### Prossimi passi

Ora che hai le basi funzionanti, puoi personalizzare l'aspetto e il comportamento nella guida alle Opzioni di Configurazione. Consulta la guida al Design Responsivo per capire come funziona il posizionamento basato su percentuali. Scopri lo stile e il supporto della modalità scura nella guida alla Personalizzazione. Per integrazioni avanzate, esplora il Riferimento API.

### Librerie frontend

Tutte le librerie frontend di FastComments (react, vue, angular, ecc.) includono Image Chat.