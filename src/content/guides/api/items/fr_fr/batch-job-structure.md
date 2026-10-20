A `BatchJob` est une tâche de longue durée, comme une importation, une exportation ou une [migration de page](#pages-migrations-post). Les travaux sont mis en file d’attente, puis récupérés et exécutés en arrière-plan.

`processedCount` et `totalCount` indiquent la progression. Leur unité dépend du `type` :

- `migrate-pages` : migrations terminées, sur le nombre de migrations demandées.
- `undo-page-migration` : enregistrements restaurés. Il n’y a pas de `totalCount`.
- `import`, `export` et autres travaux : lignes traitées.

La structure de l’objet `BatchJob` est la suivante :

[inline-code-attrs-start title = 'Structure BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJob {
    id: string
    /** Par exemple migrate-pages, undo-page-migration, migrate-domains, import, export, users-export ou copy-comments. **/
    type: string
    status: 'requested' | 'running' | 'success' | 'failed'
    createdAt: string
    completedAt?: string
    processedCount: number
    /** Travail total, lorsqu’il est connu à l’avance. **/
    totalCount?: number
    /** migrate-pages : une entrée par migration terminée, dans l’ordre de la requête. **/
    pageMigrationResults?: Array<PageMigrationSuccess | PageMigrationFailure>
    /**
     * migrate-pages : défini une fois le travail terminé (succès ou échec). Transmettre à POST /api/v1/pages/migrations/:undoId/undo dans les 90 jours.
     * undo-page-migration : la migration en cours d’annulation.
     **/
    undoId?: string
    /** undo-page-migration : défini une fois le travail terminé. **/
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
    /** Enregistrements remis à leurs valeurs d’avant la migration. **/
    restored: number
    /** Enregistrements laissés tels quels parce qu’ils ont changé après la migration, ou n’existent plus. **/
    skipped: number
    /** Enregistrements de la migration supprimés lors de la fusion des pages. Ceux-ci ne peuvent pas être restaurés. **/
    notRestorable: number
}
[inline-code-end]

#### Réessais

Si le serveur exécutant un travail de migration de page redémarre, le travail est remis dans la file d’attente et reprend après la dernière migration terminée. Un travail est tenté jusqu’à 3 fois, après quoi son `status` devient `failed`.

---