**ID del modello:** `tos_enforcer`

Il template Moderator è il punto di partenza consigliato se il tuo obiettivo è ridurre il carico di moderazione manuale. Esamina i commenti nuovi e quelli segnalati e applica le tue regole della community.

Quasi sempre vorrai **integrare il prompt incorporato** con esempi concreti di ciò che il tuo sito permette e non permette. La politica di escalation della piattaforma stessa (avvertire prima del ban, cercare nella memoria prima di bannare) è già inclusa nel prompt di sistema che l'agente riceve, quindi non è necessario ripeterla.

### Trigger

- **New comment posted** (`COMMENT_ADD`) - l'agente esamina ogni commento nuovo.
- **Comment crosses a flag threshold** (`COMMENT_FLAG_THRESHOLD`, default threshold: 3) - l'agente rivaluta un commento che altri utenti hanno segnalato.

### Strumenti consentiti

- [`mark_comment_approved`](#tools-overview) - utile per tenant in pre-moderazione dove l'agente rilascia i commenti puliti e nasconde il resto.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Non può pubblicare commenti, votare, appuntare, bloccare, assegnare badge o inviare email - il prompt è intenzionalmente ristretto.

### Aggiunte consigliate prima di andare in produzione

- **Imposta le [Linee guida della community](#community-guidelines).** Poche frasi di policy scritta sono sufficienti; l'agente le applica a ogni esecuzione.
- **Proteggi `ban_user` dietro un [approvazione](#approval-workflow).** Questo è abilitato di default nella regione UE (vedi [Conformità all'articolo 17 del DSA UE](#eu-dsa-compliance)) e raccomandato ovunque.
- **Considera anche di mettere `mark_comment_spam` dietro approvazione** se hai contenuti a bassa frequenza ma di alto rischio.
- **Proteggi `mark_comment_approved` dietro approvazione se gestisci pre-moderazione.** Approvare un commento sbagliato lo mette davanti ai lettori; proteggilo finché l'agente non si è guadagnato fiducia tramite dry-run.
- **Seleziona "Include commenter's trust factor, account age, ban history, and recent comments"** nelle [Opzioni di contesto](#context-options). Il modello avvertirà molto meno aggressivamente quando può vedere che qualcuno è un utente di lunga data in buona fede.

### Finestra consigliata per il dry-run

Esegui questo modello in [modalità dry-run](#dry-run-mode) per almeno una settimana contro il tuo traffico reale prima di passare a Enabled. Usa le [Test Runs (Replays)](#test-runs-replays) per una anteprima anche sui 30 giorni precedenti.