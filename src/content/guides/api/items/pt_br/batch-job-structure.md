Um `BatchJob` é uma tarefa de longa duração, como uma importação, uma exportação ou uma [migração de página](#pages-migrations-post). Os trabalhos são enfileirados, então capturados e executados em segundo plano.

`processedCount` e `totalCount` mostram o progresso. Sua unidade depende do `type`:

- `migrate-pages`: migrações concluídas, em relação às migrações solicitadas.
- `undo-page-migration`: registros restaurados. Não há `totalCount`.
- `import`, `export` e outros trabalhos: linhas processadas.

A estrutura do objeto `BatchJob` é a seguinte:

[inline-code-attrs-start title = 'Estrutura do BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJob {
    id: string
    /** Por exemplo migrate-pages, undo-page-migration, migrate-domains, import, export, users-export ou copy-comments. **/
    type: string
    status: 'requested' | 'running' | 'success' | 'failed'
    createdAt: string
    completedAt?: string
    processedCount: number
    /** Trabalho total, quando é conhecido antecipadamente. **/
    totalCount?: number
    /** migrate-pages: uma entrada por migração concluída, na ordem da solicitação. **/
    pageMigrationResults?: Array<PageMigrationSuccess | PageMigrationFailure>
    /**
     * migrate-pages: definido assim que o trabalho for concluído (sucesso ou falha). Passe para POST /api/v1/pages/migrations/:undoId/undo dentro de 90 dias.
     * undo-page-migration: a migração que está sendo desfeita.
     **/
    undoId?: string
    /** undo-page-migration: definido assim que o trabalho for concluído. **/
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
    /** Registros devolvidos aos seus valores de antes da migração. **/
    restored: number
    /** Registros deixados como estão porque foram alterados após a migração, ou não existem mais. **/
    skipped: number
    /** Registros da migração excluídos ao mesclar páginas. Estes não podem ser recuperados. **/
    notRestorable: number
}
[inline-code-end]

#### Tentativas

Se o servidor que executa um trabalho de migração de página for reiniciado, o trabalho é colocado de volta na fila e retoma após a última migração concluída. Um trabalho é tentado até 3 vezes, após o que seu `status` passa a ser `failed`.