Bir urlId veya domain'den diğerine yorumları, sayfaları ve ilgili kayıtları taşıyan bir toplu iş (batch job) kuyruğa alır.
"urlId" geçişleri yalnızca tam olarak eşleşen urlId ile çalışır. "domain" geçişleri her urlId ve url içindeki değeri bulur/değiştirir.
GET /api/v1/batch-jobs/{batchJobId} uç noktasını yoklayın (poll): işlem tamamlandığında geçiş başına sonuçları ve bir undoId içerir.

## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| migrateAPIPagesData | MigrateAPIPagesData | Evet |  |

## Yanıt

Döndürür: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/QueuePageMigrationJobAPIResponse.ts)

## Örnek

[inline-code-attrs-start title = 'migratePages Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'abc123def456';

const migrations: PageMigration[] = [
  {
    type: PageMigrationType.FULL_MIGRATION,
    url: 'https://myblog.com/posts/react-hooks-guide',
    title: 'The Complete Guide to React Hooks',
    comments: 'disqus-comments-export.xml',
    date: new Date('2024-01-15T10:30:00Z')
  },
  {
    type: PageMigrationType.COMMENT_ONLY,
    url: 'https://myblog.com/posts/typescript-tips',
    title: '10 Essential TypeScript Tips',
    comments: [
      { id: 'c-101', author: 'jane_doe', message: 'Great article, thanks for sharing!', date: '2024-01-16T08:12:00Z' },
      { id: 'c-102', author: 'dev_mike', message: 'Very helpful, bookmarked this one.', date: '2024-01-16T14:47:00Z' }
    ]
  }
];

const migrateAPIPagesData: MigrateAPIPagesData = {
  source: 'disqus',
  dryRun: false,
  pages: migrations
};

const response: QueuePageMigrationJobAPIResponse = await migratePages(tenantId, migrateAPIPagesData);

if (response.status === 'success') {
  console.log(`Geçiş işi kuyruğa alındı: ${migrateAPIPagesData.pages.length} sayfa için ${response.jobId}`);
}
[inline-code-end]