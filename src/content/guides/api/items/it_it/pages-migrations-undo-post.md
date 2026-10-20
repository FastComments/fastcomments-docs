[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations/:undoId/undo'; creditsCost = 1; api-resource-header-end]

Questa route annulla ogni migrazione da un lavoro [`POST /api/v1/pages/migrations`](#pages-migrations-post). Ottieni l'`undoId` dal lavoro di migrazione completato con [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id). Un `undoId` può essere usato per 90 giorni, e solo una volta.

L'annullamento viene eseguito come un batch job, allo stesso modo della migrazione, e la risposta contiene il suo `batchJobId`. Una volta completato il lavoro, il suo `pageMigrationUndoResult` riporta:

- `restored`: record riportati ai loro valori precedenti alla migrazione.
- `skipped`: record lasciati invariati perché sono cambiati dopo la migrazione, o non esistono più. Una modifica successiva non viene mai sovrascritta.
- `notRestorable`: record che la migrazione ha eliminato durante l'unione di due pagine, o rimuovendo reazioni duplicate di pagina. Questi non possono essere ripristinati.

Le migrazioni vengono annullate dall'ultima alla prima, quindi una catena come `a` a `b` seguita da `b` a `c` viene srotolata indietro fino a `a`. Un lavoro di migrazione che è fallito a metà può comunque essere annullato, il che inverte le modifiche apportate prima del fallimento.

Mettere in coda un annullamento non costa nulla. Quando il lavoro termina, costa `1` credito per `1000` record ripristinati, con un minimo di `1`.

[inline-code-attrs-start title = 'Esempio cURL Annulla Migrazioni Pagina'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations/my-undo-id/undo?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Richiesta Annulla Migrazioni Pagina'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Risposta Annulla Migrazioni Pagina'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsResponse {
    status: 'success' | 'failed'
    /**
     * Included on failure.
     * undo-not-found (404): no migration has this undoId, or it is older than 90 days.
     * already-undone (409) and undo-in-progress (409): the migration has been undone, or an undo of it is queued or running.
     **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'undo-not-found' | 'already-undone' | 'undo-in-progress' | 'internal'
    /** Included on failure. **/
    reason?: string
    /** Pass to GET /api/v1/batch-jobs/:id for progress and the result. **/
    batchJobId?: string
}
[inline-code-end]