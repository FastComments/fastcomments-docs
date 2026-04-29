Un **Agente AI** è un lavoratore autonomo, associato al tuo tenant FastComments, che monitora gli eventi nella tua community e agisce per tuo conto.

Ogni agente ha tre elementi che controlli:

1. **Una personalità.** Un prompt iniziale in testo libero che definisce tono, ruolo e stile decisionale ("Sei un caloroso accogliente della community", "Applichi le regole della community ma tendi a preferire l'ammonizione rispetto al ban", e così via).
2. **Uno o più trigger.** Una lista di eventi che attivano l'agente - un nuovo commento, un commento che supera una soglia di voti o segnalazioni, un'azione di un moderatore, il primo commento di un utente sul sito e altri. L'elenco completo è in [Trigger Events Overview](#triggers-overview).
3. **Una lista di strumenti consentiti.** Cosa l'agente è autorizzato a fare - pubblicare un commento, votare, pinnare, bloccare, contrassegnare come spam, bannare un utente, avvisare via DM, assegnare un badge, inviare email, salvare e cercare in una memoria condivisa. L'elenco completo è in [Allowed Tool Calls Overview](#tools-overview).

Quando si attiva un trigger, l'agente riceve un messaggio di contesto che descrive cosa è successo (il commento, la pagina, il contesto opzionale di thread/utente/pagina) e viene presentato con il suo prompt iniziale e le tue linee guida della community. Successivamente chiama gli strumenti per agire, registrando una giustificazione e un punteggio di confidenza ad ogni chiamata.

### Gli agenti vengono eseguiti in modo asincrono

Gli agenti **non bloccano mai l'azione dell'utente che li ha attivati**. Un lettore pubblica un commento, il commento viene salvato e mostrato nel thread, la risposta viene restituita, e solo *dopo* l'agente lo elabora - immediatamente o dopo un ritardo configurato (vedi [Deferred Triggers](#trigger-deferred-delay)). Niente di ciò che l'agente fa aggiunge latenza all'esperienza lato utente.

### Perché usarli

- **Moderare su larga scala.** Contrassegnare lo spam evidente e bannare i recidivi senza dover sorvegliare la coda 24/7.
- **Accogliere i nuovi commentatori.** Rispondere ai commentatori alla prima esperienza con la tua voce.
- **Far emergere i contenuti migliori.** Pinnare commenti di primo livello sostanziosi una volta che superano una soglia di voti.
- **Applicare le tue linee guida in modo coerente.** Applicare lo stesso testo di policy a ogni commento borderline.
- **Riassumere thread lunghi.** Pubblicare riepiloghi neutrali di dibattiti su più pagine.

### Cosa ti mantiene al controllo

- **Modalità Dry Run.** Ogni nuovo agente viene fornito in **Dry Run**: elabora i trigger, esegue il modello e registra ciò che *farebbe*, ma non compie azioni reali. Vedi [Dry-Run Mode](#dry-run-mode).
- **Approvazioni.** Qualsiasi sottoinsieme di azioni può essere soggetto ad approvazione umana. Vedi [Approval Workflow](#approval-workflow).
- **Budget per agente e per account.** Limiti rigidi giornalieri e mensili. Vedi [Budgets Overview](#budgets-overview).
- **Lista di strumenti consentiti.** Gli strumenti non consentiti vengono rimossi dalla tavolozza del modello - l'agente letteralmente non può richiederli. Vedi [Allowed Tool Calls Overview](#tools-overview).
- **Campi di audit su ogni azione.** Il modello deve includere una giustificazione e un punteggio di confidenza. Entrambi compaiono nella timeline della esecuzione e su ogni approvazione. Vedi [Run Detail View](#run-detail-view).
- **Articolo 17 della DSA UE.** Nella regione UE, i ban completamente automatizzati sono bloccati. Vedi [EU DSA Article 17 Compliance](#eu-dsa-compliance).
- **Nessun addestramento sui tuoi dati.** FastComments utilizza provider che non addestrano i loro modelli sui tuoi prompt o commenti.

### Dove si collocano rispetto alla moderazione umana

Agenti e moderatori umani condividono la stessa piattaforma di commenti: gli agenti compiono azioni attraverso gli stessi canali (approvare, contrassegnare come spam, bannare, assegnare un badge, pinnare, bloccare, scrivere) e tali azioni appaiono negli stessi [Comment Logs](/guide-moderation.html#comment-logs), nella stessa [Moderation Page](/guide-moderation.html#moderate-comments-page) e negli stessi stream di notifiche. Agenti e umani vedono il lavoro reciproco e possono reagire l'uno all'altro - le azioni dei moderatori sono esse stesse trigger validi per gli agenti (vedi [Trigger: Moderator Reviewed Comment](#trigger-moderator-reviewed) e affini).