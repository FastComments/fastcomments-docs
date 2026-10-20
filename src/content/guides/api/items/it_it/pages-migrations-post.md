[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations'; creditsCost = 1; api-resource-header-end]

Questo endpoint sposta commenti e pagine da un `urlId` o dominio a un altro, allo stesso modo dello [Domain Migration tool](/guide-migrations.html#migrating-domains) nella dashboard. È possibile inviare fino a 100 migrazioni per richiesta. Vengono eseguite nell'ordine fornito, quindi `a` a `b` seguito da `b` a `c` funziona.

Ogni migrazione ha un `type`:

- `urlId` sposta la pagina il cui `urlId` è esattamente `from`. `post-1` non corrisponde a `post-10`.
- `domain` trova/sostituisce `from` all'interno di ogni `urlId` e `url`, e aggiorna il `domain` di ogni commento. Entrambi i valori devono essere un dominio, ad esempio `old.example.com`.

Commenti, la pagina, le reazioni alla pagina, i voti, le notifiche e le sottoscrizioni vengono tutti spostati. Se una pagina esiste già su `to`, le due pagine vengono unite.

La migrazione viene eseguita come un batch job. La risposta contiene un `batchJobId`. Esegui il polling di [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id) per seguirne l'avanzamento. Una volta completato il job, contiene un risultato per migrazione e un `undoId` che puoi passare a [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post) entro 90 giorni.

La richiesta viene convalidata prima che venga accodata. Se una migrazione è invalida, l'intera richiesta fallisce con un `422` e nulla viene eseguito.

Accodare una migrazione non costa nulla. Quando il job termina, costa `1` credito per `1000` commenti migrati, con un minimo di `1`.

[inline-code-attrs-start title = 'Esempio cURL per le migrazioni di pagina'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"migrations": [
		{"from": "old-post-id", "to": "new-post-id", "type": "urlId"},
		{"from": "old.example.com", "to": "new.example.com", "type": "domain"}
	]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta di migrazioni di pagina'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}

interface PageMigrationsBody {
    /** Run in the order given. At most 100. **/
    migrations: PageMigration[]
}

interface PageMigration {
    /** At most 1000 characters. Leading and trailing whitespace is removed. **/
    from: string
    /** Must be different from "from". At most 1000 characters. **/
    to: string
    /** "urlId" only matches the exact urlId. "domain" find/replaces the value inside every urlId and url. **/
    type: 'urlId' | 'domain'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta di migrazioni di pagina'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-migrations' | 'too-many-migrations' | 'invalid-migration' | 'internal'
    /** Included on failure. For invalid-migration, this names the index of the first invalid migration. **/
    reason?: string
    /** Pass to GET /api/v1/batch-jobs/:id for progress and the result. **/
    batchJobId?: string
}
[inline-code-end]

#### Note

- I job per un tenant vengono eseguiti uno alla volta, dal più vecchio al più recente. Una seconda migrazione attende che la prima termini.
- `PATCH /api/v1/pages/:id` con un nuovo `urlId` cambia solo la pagina, non i suoi commenti. Usa questo endpoint per spostare i commenti di una pagina.