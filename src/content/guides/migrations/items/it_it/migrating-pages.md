---
Sebbene il suo nome sia lo strumento di migrazione del dominio, può anche essere usato per migrare i commenti tra pagine individuali.

Inserisci semplicemente i valori vecchi e nuovi di [urlId](/guide-customizations-and-configuration.html#url-id) nei campi di input `from` e `to`.

### Migrazione delle pagine tramite API

Per migrare molte pagine contemporaneamente, usa [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post). Accetta fino a 100 coppie `from` e `to` per richiesta e le esegue come un lavoro batch.

Con `type` impostato su `urlId`, solo la pagina il cui `urlId` è esattamente `from` viene spostata, quindi `post-1` non corrisponde a `post-10`. Lo strumento della dashboard non ha questa opzione.

Ogni migrazione API può essere annullata per 90 giorni con [`POST /api/v1/pages/migrations/:undoId/undo`](/guide-api.html#pages-migrations-undo-post).

---