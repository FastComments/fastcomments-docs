Un `BatchJob` è un'attività a lunga esecuzione, come un'importazione, un'esportazione o una [migrazione di pagina](#pages-migrations-post). I lavori sono accodati, poi prelevati ed eseguiti in background.

`processedCount` e `totalCount` mostrano l'avanzamento. La loro unità dipende dal `type`:

- `migrate-pages`: migrazioni completate, rispetto alle migrazioni richieste.
- `undo-page-migration`: record ripristinati. Non c'è `totalCount`.
- `import`, `export` e altri lavori: righe elaborate.

La struttura dell'oggetto `BatchJob` è la seguente:

[inline-code-attrs-start title = 'Struttura BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJob {
    id: string
    /** Ad esempio migrate-pages, undo-page-migration, migrate-domains, import, export, users-export o copy-comments. **/
    type: string
    status: 'requested' | 'running' | 'success' | 'failed'
    createdAt: string
    completedAt?: string
    processedCount: number
    /** Lavoro totale, quando è noto in anticipo. **/
    totalCount?: number
    /** migrate-pages: una voce per ogni migrazione completata, in ordine di richiesta. **/
    pageMigrationResults?: Array<PageMigrationSuccess | PageMigrationFailure>
    /**
     * migrate-pages: impostato una volta che il lavoro è terminato (successo o fallito). Passare a POST /api/v1/pages/migrations/:undoId/undo entro 90 giorni.
     * undo-page-migration: la migrazione da annullare.
     **/
    undoId?: string
    /** undo-page-migration: impostato una volta che il lavoro è terminato. **/
    pageMigrationUndoResult?: PageMigrationUndoResult
}

interface PageMigrationSuccess {
    from: string
    to: string
    type: 'urlId' | 'domain'
    status: 'success'
    commentsMigrated: number
}

interface PageMigrationFailure {
    from: string
    to: string
    type: 'urlId' | 'domain'
    status: 'failed'
    code: string
    reason: string
}

interface PageMigrationUndoResult {
    /** Record ripristinati ai loro valori precedenti la migrazione. **/
    restored: number
    /** Record lasciati invariati perché sono cambiati dopo la migrazione, o non esistono più. **/
    skipped: number
    /** Record della migrazione eliminati durante l'unione delle pagine. Questi non possono essere ripristinati. **/
    notRestorable: number
}
[inline-code-end]

#### Tentativi

Se il server che esegue un lavoro di migrazione di pagina si riavvia, il lavoro viene reinserito nella coda e riprende dopo l'ultima migrazione completata. Un lavoro viene provato fino a 3 volte, dopodiché il suo `status` è `failed`.

---