A `BatchJob` bir içe aktarma, bir dışa aktarma veya bir [page migration](#pages-migrations-post) gibi uzun süren bir görevdir. İşler kuyruğa alınır, ardından alınır ve arka planda çalıştırılır.

`processedCount` ve `totalCount` ilerlemeyi gösterir. Birimleri `type` değerine bağlıdır:

- `migrate-pages`: istenen taşıma sayısına göre tamamlanan taşımalar.
- `undo-page-migration`: geri getirilen kayıtlar. `totalCount` yoktur.
- `import`, `export` ve diğer işler: işlenen satırlar.

`BatchJob` nesnesinin yapısı aşağıdaki gibidir:

[inline-code-attrs-start title = 'BatchJob Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJob {
    id: string
    /** Örneğin migrate-pages, undo-page-migration, migrate-domains, import, export, users-export veya copy-comments. **/
    type: string
    status: 'requested' | 'running' | 'success' | 'failed'
    createdAt: string
    completedAt?: string
    processedCount: number
    /** Önceden bilindiğinde toplam iş. **/
    totalCount?: number
    /** migrate-pages: istek sırasına göre tamamlanan her taşıma için bir giriş. **/
    pageMigrationResults?: Array<PageMigrationSuccess | PageMigrationFailure>
    /**
     * migrate-pages: iş tamamlandığında (başarılı veya başarısız) ayarlanır. 90 gün içinde POST /api/v1/pages/migrations/:undoId/undo adresine gönderin.
     * undo-page-migration: geri alınan taşıma.
     **/
    undoId?: string
    /** undo-page-migration: iş tamamlandığında ayarlanır. **/
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
    /** Taşıma öncesi değerlerine geri getirilen kayıtlar. **/
    restored: number
    /** Taşıma sonrası değiştiği için veya artık mevcut olmadığı için bırakılan kayıtlar. **/
    skipped: number
    /** Sayfalar birleştirilirken silinen taşıma kayıtları. Bunlar geri getirilemez. **/
    notRestorable: number
}
[inline-code-end]

#### Yeniden Denemeler

Sayfa taşıma işi çalıştıran sunucu yeniden başlatılırsa, iş kuyruğa geri konur ve son tamamlanan taşımanın ardından devam eder. Bir iş en fazla 3 kez denenir, ardından `status` değeri `failed` olur.