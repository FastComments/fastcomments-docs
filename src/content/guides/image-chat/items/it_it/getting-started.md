### Casi d'uso

Image Chat è ideale per il feedback di design quando i team devono discutere elementi specifici in mockup o screenshot. I siti di recensioni di prodotti possono permettere ai clienti di discutere caratteristiche specifiche visibili nelle foto dei prodotti. Le piattaforme educative possono usarlo per discutere diagrammi, mappe o immagini scientifiche. Le gallerie fotografiche possono diventare interattive con commenti legati a posizioni specifiche. I siti immobiliari possono permettere ai visitatori di discutere caratteristiche specifiche visibili nelle foto delle proprietà.

### Quick Start

Per iniziare con Image Chat è semplice. Hai bisogno dello script FastComments Image Chat, di un elemento immagine o di un contenitore con un'immagine e di un oggetto di configurazione con il tuo Tenant ID.

### Installazione

Aggiungi lo script Image Chat alla tua pagina:

[inline-code-attrs-start title = 'Caricamento dello script di Image Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
[inline-code-end]

### Implementazione di base

Ecco un esempio minimo:

[inline-code-attrs-start title = 'Implementazione di base di Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Image Gallery with Image Chat</title>
</head>
<body>
    <!-- La tua immagine -->
    <img id="my-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Pretty Trail" />

    <!-- Carica lo script Image Chat -->
    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>

    <!-- Inizializza Image Chat -->
    <script>
        FastCommentsImageChat(document.getElementById('my-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Sostituisci `'demo'` con il tuo reale FastComments Tenant ID se non lo è già, che puoi trovare nel tuo [dashboard di FastComments](https://fastcomments.com/auth/my-account).

### Come funziona

Una volta inizializzato, gli utenti possono cliccare in qualsiasi punto dell'immagine. Quando avviene un clic, un marcatore quadrato visivo appare in quella posizione e si apre una finestra di chat. Altri utenti possono vedere tutti i marcatori e cliccarli per visualizzare o partecipare a quelle discussioni. Tutte le discussioni si sincronizzano in tempo reale tra tutti i visitatori.

Il widget utilizza un posizionamento basato su percentuali, quindi i marcatori rimangono nella posizione corretta anche quando l'immagine viene ridimensionata o viene visualizzata su schermi di dimensioni diverse.

### Demo dal vivo

Puoi vedere Image Chat in azione nella nostra [pagina demo dal vivo](https://fastcomments.com/product/image-chat).

### Prossimi passi

Ora che hai le basi funzionanti, puoi personalizzare l'aspetto e il comportamento nella guida alle Opzioni di Configurazione. Consulta la guida al Design Responsivo per capire come funziona il posizionamento basato su percentuali. Scopri lo styling e il supporto per la modalità scura nella guida alla Personalizzazione. Per integrazioni avanzate, esplora il Riferimento API.

### Librerie Frontend

Tutte le librerie frontend di FastComments (react, vue, angular, ecc.) includono Image Chat.