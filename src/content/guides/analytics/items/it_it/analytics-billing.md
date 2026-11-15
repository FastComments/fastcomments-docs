Tutte le metriche per cui potresti essere soggetto a limitazioni di frequenza o addebiti vengono riportate nella [Pagina di Analisi della Fatturazione](https://fastcomments.com/auth/my-account/analytics/billing).

Alcune metriche di esempio che vengono tracciate:

- Caricamenti di Pagina (per il mese corrente)
- Crediti API (per il mese corrente)
- Nuovi Commenti Memorizzati (per il mese corrente)
- Utenti Tenant
- Utenti SSO (utenti SSO regolari senza permessi di amministratore/moderatore)
- Amministratori SSO (utenti SSO con flag `isAccountOwner` o `isAdminAdmin` - fatturati separatamente dagli utenti SSO regolari)
- Moderatori SSO (utenti SSO con flag `isCommentModeratorAdmin` - fatturati separatamente dagli utenti SSO regolari)
- Moderatori
- Domini

Per i piani `Creators` e `Pro`, queste metriche servono semplicemente come guida per sapere se sei vicino o hai raggiunto i limiti del tuo account.

Per il piano `Flex`, questa pagina mostrerà una ripartizione dei costi dell'utilizzo attuale per la fattura **successiva**. Questo rende facile capire come verrai fatturato prima che accada. Puoi vedere le fatture passate in [Cronologia Fatturazione](https://fastcomments.com/auth/my-account/billing-history). Come viene conteggiata ogni voce e quando viene addebitata la fattura è spiegato nella [Guida alla Fatturazione](/guide-billing.html#how-flex-usage-is-measured).

Le metriche `Page Loads`, `API Credits` e `New Comments Stored` potrebbero essere in ritardo fino a un minuto. Le altre metriche sono in tempo reale.

Queste informazioni vengono memorizzate su base mensile per un anno, o fino a quando non elimini il tuo tenant, per scopi storici, ma l'interfaccia utente mostrerà solo il periodo corrente.