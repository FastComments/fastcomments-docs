하나의 urlId 또는 도메인에서 다른 곳으로 댓글, 페이지 및 관련 레코드를 이동하는 배치 작업을 큐에 추가합니다.
"urlId" 마이그레이션은 정확히 일치하는 urlId만 대상으로 합니다. "domain" 마이그레이션은 모든 urlId와 url 내에서 값을 찾아 교체합니다.
GET /api/v1/batch-jobs/{batchJobId}를 폴링하세요: 완료되면 마이그레이션별 결과와 undoId를 포함합니다.

## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| migrateAPIPagesData | MigrateAPIPagesData | 예 |  |

## 응답

반환값: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/QueuePageMigrationJobAPIResponse.ts)

## 예제

[inline-code-attrs-start title = 'migratePages 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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