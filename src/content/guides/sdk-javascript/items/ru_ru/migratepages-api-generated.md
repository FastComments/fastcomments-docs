Ставит в очередь пакетное задание, которое переносит комментарии, страницы и связанные записи с одного urlId или домена на другой.
Миграции "urlId" сопоставляют только точный urlId. Миграции "domain" выполняют поиск/замену значения внутри каждого urlId и url.
Опрашивайте GET /api/v1/batch-jobs/{batchJobId}: после завершения задание содержит результаты по каждой миграции и undoId.

## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| migrateAPIPagesData | MigrateAPIPagesData | Да |  |

## Ответ

Возвращает: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/QueuePageMigrationJobAPIResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример migratePages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
  console.log(`Задание миграции поставлено в очередь: ${response.jobId} для ${migrateAPIPagesData.pages.length} страниц`);
}
[inline-code-end]