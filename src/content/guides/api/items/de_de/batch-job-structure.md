Ein `BatchJob` ist eine lang laufende Aufgabe, wie ein Import, ein Export oder eine [Seitenmigration](#pages-migrations-post). Jobs werden in eine Warteschlange gestellt, dann abgeholt und im Hintergrund ausgeführt.

`processedCount` und `totalCount` zeigen den Fortschritt an. Ihre Einheit hängt vom `type` ab:

- `migrate-pages`: abgeschlossene Migrationen, von den angeforderten Migrationen.
- `undo-page-migration`: wiederhergestellte Datensätze. Es gibt keinen `totalCount`.
- `import`, `export` und andere Jobs: verarbeitete Zeilen.

Die Struktur des `BatchJob`-Objekts ist wie folgt:

[inline-code-attrs-start title = 'BatchJob Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJob {
    id: string
    /** Zum Beispiel migrate-pages, undo-page-migration, migrate-domains, import, export, users-export oder copy-comments. **/
    type: string
    status: 'requested' | 'running' | 'success' | 'failed'
    createdAt: string
    completedAt?: string
    processedCount: number
    /** Gesamtarbeit, wenn sie im Voraus bekannt ist. **/
    totalCount?: number
    /** migrate-pages: ein Eintrag pro abgeschlossener Migration, in Anforderungsreihenfolge. **/
    pageMigrationResults?: Array<PageMigrationSuccess | PageMigrationFailure>
    /**
     * migrate-pages: wird gesetzt, sobald der Job beendet ist (Erfolg oder Fehlschlag). Weitergeben an POST /api/v1/pages/migrations/:undoId/undo innerhalb von 90 Tagen.
     * undo-page-migration: die rückgängig zu machende Migration.
     **/
    undoId?: string
    /** undo-page-migration: wird gesetzt, sobald der Job beendet ist. **/
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
    /** Datensätze, die auf ihre Werte vor der Migration zurückgesetzt wurden. **/
    restored: number
    /** Datensätze, die unverändert bleiben, weil sie nach der Migration geändert wurden oder nicht mehr existieren. **/
    skipped: number
    /** Datensätze, die bei der Zusammenführung von Seiten gelöscht wurden. Diese können nicht wiederhergestellt werden. **/
    notRestorable: number
}
[inline-code-end]

#### Wiederholungen

Wenn der Server, der einen Seitenmigrations-Job ausführt, neu startet, wird der Job zurück in die Warteschlange gestellt und nach der letzten abgeschlossenen Migration wieder aufgenommen. Ein Job wird bis zu 3 Mal versucht, danach ist sein `status` `failed`.

---