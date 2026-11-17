將批次工作排入佇列，將留言、頁面及相關記錄從一個 urlId 或網域遷移至另一個。
"urlId" 遷移僅比對完全相同的 urlId。"domain" 遷移會在每個 urlId 與 url 中進行尋找/取代。
輪詢 GET /api/v1/batch-jobs/{batchJobId}：完成後會包含每次遷移的結果與 undoId。

## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| migrateAPIPagesData | MigrateAPIPagesData | 是 |  |

## 回應

回傳：[`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/QueuePageMigrationJobAPIResponse.ts)

## 範例

[inline-code-attrs-start title = 'migratePages 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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