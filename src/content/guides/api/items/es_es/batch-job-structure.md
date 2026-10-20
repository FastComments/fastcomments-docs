Un `BatchJob` es una tarea de larga duración, como una importación, una exportación o una [migración de página](#pages-migrations-post). Los trabajos se ponen en cola, luego se recogen y se ejecutan en segundo plano.

`processedCount` y `totalCount` muestran el progreso. Su unidad depende del `type`:

- `migrate-pages`: migraciones finalizadas, de las migraciones solicitadas.
- `undo-page-migration`: registros restaurados. No hay `totalCount`.
- `import`, `export` y otros trabajos: filas procesadas.

La estructura del objeto `BatchJob` es la siguiente:

[inline-code-attrs-start title = 'Estructura de BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJob {
    id: string
    /** Por ejemplo migrate-pages, undo-page-migration, migrate-domains, import, export, users-export o copy-comments. **/
    type: string
    status: 'requested' | 'running' | 'success' | 'failed'
    createdAt: string
    completedAt?: string
    processedCount: number
    /** Trabajo total, cuando se conoce de antemano. **/
    totalCount?: number
    /** migrate-pages: una entrada por cada migración finalizada, en orden de solicitud. **/
    pageMigrationResults?: Array<PageMigrationSuccess | PageMigrationFailure>
    /**
     * migrate-pages: se establece una vez que el trabajo ha finalizado (éxito o fallido). Pasar a POST /api/v1/pages/migrations/:undoId/undo dentro de los 90 días.
     * undo-page-migration: la migración que se está deshaciendo.
     **/
    undoId?: string
    /** undo-page-migration: se establece una vez que el trabajo ha finalizado. **/
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
    /** Registros restaurados a sus valores anteriores a la migración. **/
    restored: number
    /** Registros dejados sin cambios porque se modificaron después de la migración, o ya no existen. **/
    skipped: number
    /** Registros de la migración eliminados al combinar páginas. No pueden ser recuperados. **/
    notRestorable: number
}
[inline-code-end]

#### Reintentos

Si el servidor que ejecuta un trabajo de migración de página se reinicia, el trabajo se vuelve a colocar en la cola y se reanuda después de la última migración finalizada. Un trabajo se intenta hasta 3 veces, después de lo cual su `status` es `failed`.