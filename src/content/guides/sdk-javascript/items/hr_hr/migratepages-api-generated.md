Postavlja batch posao u red čekanja koji premješta komentare, stranice i povezane zapise s jednog urlId ili domene na drugi.
"urlId" migracije podudaraju samo točan urlId. "domain" migracije pronalaze/zamjenjuju vrijednost unutar svakog urlId i url.
Pitajte GET /api/v1/batch-jobs/{batchJobId}: kada je gotov, sadrži rezultate po migraciji i undoId.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| migrateAPIPagesData | MigrateAPIPagesData | Da |  |

## Odgovor

Vraća: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/QueuePageMigrationJobAPIResponse.ts)

## Primjer

[inline-code-attrs-start title = 'migratePages primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
  console.log(`Queued migration job: ${response.jobId} for ${migrateAPIPagesData.pages.length} pages`);
}
[inline-code-end]