Een `BatchJob` is een langdurige taak, zoals een import, een export, of een [pagina-migratie](#pages-migrations-post). Taken worden in de wachtrij geplaatst, vervolgens opgepakt en op de achtergrond uitgevoerd.

`processedCount` en `totalCount` tonen de voortgang. Hun eenheid hangt af van het `type`:

- `migrate-pages`: voltooide migraties, van de aangevraagde migraties.
- `undo-page-migration`: records hersteld. Er is geen `totalCount`.
- `import`, `export` en andere taken: verwerkte rijen.

De structuur voor het `BatchJob`-object is als volgt:

[inline-code-attrs-start title = 'BatchJob-structuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJob {
    id: string
    /** Bijvoorbeeld migrate-pages, undo-page-migration, migrate-domains, import, export, users-export of copy-comments. **/
    type: string
    status: 'requested' | 'running' | 'success' | 'failed'
    createdAt: string
    completedAt?: string
    processedCount: number
    /** Totale hoeveelheid werk, wanneer deze van tevoren bekend is. **/
    totalCount?: number
    /** migrate-pages: één entry per voltooide migratie, in volgorde van verzoek. **/
    pageMigrationResults?: Array<PageMigrationSuccess | PageMigrationFailure>
    /**
     * migrate-pages: wordt ingesteld zodra de taak is voltooid (succes of mislukt). Verstuur naar POST /api/v1/pages/migrations/:undoId/undo binnen 90 dagen.
     * undo-page-migration: de migratie die ongedaan wordt gemaakt.
     */
    undoId?: string
    /** undo-page-migration: wordt ingesteld zodra de taak is voltooid. **/
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
    /** Records teruggezet naar hun waarden van vóór de migratie. **/
    restored: number
    /** Records ongemoeid gelaten omdat ze na de migratie zijn gewijzigd, of niet meer bestaan. **/
    skipped: number
    /** Records van de migratie die zijn verwijderd tijdens het samenvoegen van pagina's. Deze kunnen niet worden teruggehaald. **/
    notRestorable: number
}
[inline-code-end]

#### Retries

Als de server die een pagina-migratietaak uitvoert opnieuw wordt opgestart, wordt de taak terug in de wachtrij geplaatst en hervat na de laatste voltooide migratie. Een taak wordt tot 3 keer geprobeerd, waarna de `status` `failed` is.