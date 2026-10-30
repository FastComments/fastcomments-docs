An **AI Agent** è un lavoratore autonomo, limitato al tuo tenant FastComments, che osserva gli eventi nella tua community e agisce per tuo conto.

Ogni agente ha tre elementi che controlli:

1. **Una personalità.** Un prompt iniziale in testo libero che definisce tono, ruolo e stile decisionale ("Sei un accogliente saluto della community", "Fai rispettare le regole della community ma tendi a dare avvertimenti piuttosto che bannare", ecc.).
2. **Uno o più trigger.** Un elenco di eventi che attivano l'agente – un nuovo commento, un commento che supera una soglia di voti o segnalazioni, un'azione di moderatore, il primo commento di un utente sul sito, e altri. L'elenco completo è in [Trigger Events Overview](#triggers-overview).
3. **Una lista di strumenti consentiti.** Ciò che l'agente può fare – pubblicare un commento, votare, fissare, bloccare, contrassegnare come spam, bannare un utente, avvisare via DM, assegnare un badge, inviare email, salvare e cercare una memoria condivisa. L'elenco completo è in [Allowed Tool Calls Overview](#tools-overview).

Quando un trigger scatta, l'agente riceve un messaggio di contesto che descrive ciò che è accaduto (il commento, la pagina, contesto opzionale di thread/utente/pagina) e viene alimentato con il suo prompt iniziale e le linee guida della tua community. Quindi chiama gli strumenti per agire, registrando una giustificazione e un punteggio di confidenza ad ogni chiamata.

### Gli agenti funzionano in modo asincrono

Gli agenti **non bloccano mai l'azione dell'utente che li ha attivati**. Un lettore pubblica un commento, il commento viene salvato e mostrato al thread, la risposta viene restituita, e solo *poi* l'agente lo elabora – immediatamente o dopo un ritardo configurato (vedi [Deferred Triggers](#trigger-deferred-delay)). Nulla di ciò che fa l'agente aggiunge latenza all'esperienza dell'utente.

### Perché usarli

- **Moderare su larga scala.** Segnare lo spam evidente e bannare i trasgressori ricorrenti senza dover monitorare la coda 24 ore su 24.
- **Accogliere i nuovi commentatori.** Rispondere ai commentatori per la prima volta con la tua voce.
- **Mettere in evidenza i contenuti migliori.** Fissare i commenti di livello superiore sostanziali una volta che superano una soglia di voti.
- **Applicare le tue linee guida in modo coerente.** Applicare lo stesso testo di policy a ogni commento borderline.
- **Riassumere thread lunghi.** Pubblicare riassunti neutrali di dibattiti su più pagine.

### Cosa ti mantiene al controllo

- **Modalità dry-run.** Ogni nuovo agente viene fornito in **Dry Run**: elabora i trigger, esegue il modello e registra ciò che *farebbe*, ma non compie azioni reali. Vedi [Dry-Run Mode](#dry-run-mode).
- **Approvazioni.** Qualsiasi sottoinsieme di azioni può essere soggetto a approvazione umana. Vedi [Approval Workflow](#approval-workflow).
- **Budget per agente e per account.** Limiti giornalieri e mensili rigidi. Vedi [Budgets Overview](#budgets-overview).
- **Lista di strumenti consentiti.** Gli strumenti non consentiti vengono rimossi dalla palette del modello – l'agente letteralmente non può richiederli. Vedi [Allowed Tool Calls Overview](#tools-overview).
- **Campi di audit su ogni azione.** Il modello deve includere una giustificazione e un punteggio di confidenza. Entrambi appaiono nella timeline di esecuzione e su ogni approvazione. Vedi [Run Detail View](#run-detail-view).
- **EU DSA Articolo 17.** Nella regione UE, i ban completamente automatizzati sono bloccati. Vedi [EU DSA Article 17 Compliance](#eu-dsa-compliance).
- **Nessun addestramento sui tuoi dati.** FastComments utilizza fornitori che non addestrano sui tuoi prompt o commenti.

### Dove si collocano accanto alla moderazione umana

Gli agenti e i moderatori umani condividono la stessa piattaforma di commenti: gli agenti compiono azioni attraverso gli stessi canali (approva, spam, banna, badge, fissa, blocca, scrivi) e tali azioni appaiono negli stessi [Comment Logs](/guide-moderation.html#comment-logs), nella stessa [Moderation Page](/guide-moderation.html#moderate-comments-page) e negli stessi flussi di notifica. Agenti e umani vedono il lavoro dell'altro e possono reagire reciprocamente – le azioni dei moderatori sono esse stesse trigger validi per gli agenti (vedi [Trigger: Moderator Reviewed Comment](#trigger-moderator-reviewed) e correlati).

Se hai bisogno anche di persone nella coda, FastComments offre [Human Moderation Services](/guide-moderation.html#human-moderation-services): i nostri moderatori lavorano accanto ai tuoi agenti, nella tua dashboard, secondo le tue linee guida.