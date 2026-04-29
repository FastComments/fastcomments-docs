**ID del template:** `tos_enforcer`

Il modello Moderatore è il punto di partenza consigliato se il tuo obiettivo è ridurre il carico di moderazione manuale. Esamina i nuovi commenti e quelli segnalati e applica le regole della tua community.

### Prompt iniziale integrato

[inline-code-attrs-start title = 'Prompt iniziale del modello Moderatore'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a terms-of-service enforcement agent. Review each new comment against the community guidelines. Mark clear spam or policy violations. Issue warnings for first-offense borderline content. Escalate ban decisions only for repeat or severe violations. If a comment is clearly within the rules, approve it so it becomes visible (relevant for pre-moderation tenants). Stay out of political or subjective debates, focus on the rules as written.
[inline-code-end]

Vorrai quasi sempre **arricchire questo prompt** con esempi concreti di ciò che il tuo sito permette e non permette. La politica di escalation della piattaforma (avvertire prima di bannare, cercare nella memoria prima di bannare) è già incorporata nel prompt di sistema che l'agente riceve, quindi non è necessario ripeterla.

### Attivatori

- **Nuovo commento pubblicato** (`COMMENT_ADD`) - l'agente esamina ogni nuovo commento.
- **Un commento supera una soglia di segnalazioni** (`COMMENT_FLAG_THRESHOLD`, soglia predefinita: 3) - l'agente rivaluta un commento che altri utenti hanno segnalato.

### Strumenti consentiti

- [`mark_comment_approved`](#tools-overview) - utile per tenant con pre-moderazione dove l'agente rilascia i commenti puliti e nasconde il resto.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Non può pubblicare commenti, votare, fissare, bloccare, assegnare badge o inviare email - il prompt è intenzionalmente ristretto.

### Aggiunte consigliate prima di andare live

- **Imposta le [Linee guida della community](#community-guidelines).** Poche frasi di policy scritta sono sufficienti; l'agente le applicherà a ogni esecuzione.
- **Metti `ban_user` dietro [approvazione](#approval-workflow).** Questo è attivo di default nella regione UE (vedi [Conformità all'articolo 17 del DSA UE](#eu-dsa-compliance)) ed è raccomandato ovunque.
- **Valuta anche di mettere `mark_comment_spam` dietro approvazione** se hai contenuti a basso volume ma di alto rischio.
- **Metti `mark_comment_approved` dietro approvazione se esegui la pre-moderazione.** Approvare un commento sbagliato lo mette davanti ai lettori; mettilo sotto controllo finché l'agente non si è guadagnato fiducia tramite una modalità di prova.
- **Seleziona "Include commenter's trust factor, account age, ban history, and recent comments"** nelle [Opzioni di contesto](#context-options). Il modello avvertirà molto meno aggressivamente quando può vedere che qualcuno è un utente di lunga data in buona fede.

### Periodo di prova consigliato

Esegui questo template in [dry-run](#dry-run-mode) per almeno una settimana contro il tuo traffico reale prima di passare a Abilitato. Usa le [Esecuzioni di test (Replay)](#test-runs-replays) per una anteprima anche sugli ultimi 30 giorni.