[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations/:undoId/undo'; creditsCost = 1; api-resource-header-end]

Bu rota, bir [`POST /api/v1/pages/migrations`](#pages-migrations-post) işinden yapılan tüm göçleri tersine çevirir. Tamamlanmış göç işinden [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id) ile `undoId` değerini alın. Bir `undoId` 90 gün boyunca ve yalnızca bir kez kullanılabilir.

Geri alma, göç gibi bir toplu iş (batch job) olarak çalışır ve yanıt, onun `batchJobId` değerini içerir. İş tamamlandığında, `pageMigrationUndoResult` şu bilgileri raporlar:

- `restored`: göçten önceki değerlerine geri döndürülen kayıtlar.
- `skipped`: göçten sonra değiştiği veya artık mevcut olmayan kayıtlar bırakılır. Daha sonraki bir düzenleme asla üzerine yazılmaz.
- `notRestorable`: iki sayfa birleştirilirken veya yinelenen sayfa tepkileri kaldırılırken göç tarafından silinen kayıtlar. Bunlar geri getirilemez.

Göçler, en yenisinden başlayarak tersine çevrilir, bu yüzden `a`'dan `b`'ye, ardından `b`'den `c`'ye olan bir zincir `a`'ya geri sarılır. Kısmen başarısız olan bir göç işi bile geri alınabilir; bu, başarısız olmadan önce yaptığı değişiklikleri tersine çevirir.

Geri almayı kuyruğa eklemek ücretsizdir. İş tamamlandığında, geri getirilen her `1000` kayıt için `1` kredi, minimum `1` kredi ücretlendirilir.

[inline-code-attrs-start title = 'Sayfa Göçlerini Geri Alma cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations/my-undo-id/undo?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Sayfa Göçlerini Geri Alma İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Sayfa Göçlerini Geri Alma Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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