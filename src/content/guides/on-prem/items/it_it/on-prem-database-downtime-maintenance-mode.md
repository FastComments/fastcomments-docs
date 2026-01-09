---
FastComments supporta una modalità di manutenzione automatica. Se il database va offline, può continuare a servire i thread di commenti più popolari.

Inoltre, in modalità di manutenzione, tutti i commenti vengono salvati in `BACKUP_DIR`. Verranno elaborati (controllati per spam, ecc.) e salvati una volta che il sistema sarà nuovamente online.

Lo fa determinando, ogni ora, i 100 thread di commenti più popolari e memorizzando la loro cache su disco. La determinazione dei 100 thread
viene già effettuata a partire da uno stato pre-calcolato, quindi non è un lavoro periodico pesante.

Questo è completamente opzionale ed è attivato solo se `CACHE_DIR` e `BACKUP_DIR` sono impostati. Questo, ovviamente, rende i nodi dell'applicazione con stato, tuttavia si tratta di uno stato che
può essere perso in qualsiasi momento senza causare malfunzionamenti dell'applicazione.

Nota che in modalità di manutenzione non è possibile eseguire correttamente l'autenticazione dei thread di commenti, quindi vengono eseguiti backup periodici solo dei thread che possono essere considerati pubblici in modo sicuro.

In modalità di manutenzione molte funzionalità non sono disponibili.

---