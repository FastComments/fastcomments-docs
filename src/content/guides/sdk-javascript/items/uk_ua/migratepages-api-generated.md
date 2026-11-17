Ставить у чергу пакетне завдання, яке переміщує коментарі, сторінки та пов'язані записи з одного urlId або домену на інший.
Міграції "urlId" збігаються лише з точним urlId. Міграції "domain" виконують пошук/заміну значення всередині кожного urlId та url.
Опитуйте GET /api/v1/batch-jobs/{batchJobId}: коли завдання завершиться, воно міститиме результати кожної міграції та undoId.

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| migrateAPIPagesData | MigrateAPIPagesData | Так |  |

## Відповідь

Повертає: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/QueuePageMigrationJobAPIResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад migratePages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
  console.log(`Завдання міграції в черзі: ${response.jobId} для ${migrateAPIPagesData.pages.length} сторінок`);
}
[inline-code-end]