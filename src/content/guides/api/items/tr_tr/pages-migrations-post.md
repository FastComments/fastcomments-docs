[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations'; creditsCost = 1; api-resource-header-end]

Bu rota, yorumları ve sayfaları bir `urlId` veya alan adından diğerine taşır, kontrol panelindeki [Domain Migration tool](/guide-migrations.html#migrating-domains) ile aynı şekilde. İstek başına en fazla 100 göç gönderilebilir. Verilen sırayla çalışırlar, yani `a`'dan `b`'ye ardından `b`'den `c`'ye geçiş çalışır.

Her göçün bir `type`'ı vardır:

- `urlId` tam olarak verilen `urlId`'ye sahip sayfayı taşır. `post-1` `post-10` ile eşleşmez.
- `domain` her `urlId` ve `url` içinde `from` değerini bulur/değiştirir ve her yorumun `domain`'ini günceller. Her iki değer de bir alan adı olmalıdır, örneğin `old.example.com`.

Yorumlar, sayfa, sayfa tepkileri, oylar, bildirimler ve abonelikler hepsi taşınır. `to` konumunda zaten bir sayfa varsa, iki sayfa birleştirilir.

Göç bir toplu iş (batch job) olarak çalışır. Yanıt bir `batchJobId` içerir. İlerlemesini izlemek için [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id) isteği gönderin. İş tamamlandığında her göç için bir sonuç ve 90 gün içinde [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post) adresine geçirebileceğiniz bir `undoId` bulunur.

İstek, kuyruğa alınmadan önce doğrulanır. Herhangi bir göç geçersizse, tüm istek `422` hatasıyla başarısız olur ve hiçbir şey çalışmaz.

Bir göçün kuyruğa alınması ücretsizdir. İş tamamlandığında, taşınan her `1000` yorum için `1` kredi, minimum `1` kredi ücretlendirilir.

[inline-code-attrs-start title = 'Sayfa Göçleri cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Sayfa Göçleri İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Sayfa Göçleri Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

#### Notlar

- Bir kiracı için işler bir seferde bir çalışır, en eski önce. İkinci bir göç, birincisinin bitmesini bekler.
- Yeni bir `urlId` ile `PATCH /api/v1/pages/:id` sadece sayfayı değiştirir, yorumlarını değil. Bir sayfanın yorumlarını taşımak için bu rotayı kullanın.

---