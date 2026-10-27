[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations'; creditsCost = 1; api-resource-header-end]

이 라우트는 댓글과 페이지를 하나의 `urlId` 또는 도메인에서 다른 것으로 이동합니다. 대시보드의 [Domain Migration tool](/guide-migrations.html#migrating-domains)와 동일합니다. 요청당 최대 100개의 마이그레이션을 보낼 수 있습니다. 지정된 순서대로 실행되므로 `a`에서 `b`로, 그 다음 `b`에서 `c`로 이동하는 것이 작동합니다.

각 마이그레이션은 `type`을 가집니다:

- `urlId`는 `urlId`가 정확히 `from`인 페이지를 이동합니다. `post-1`은 `post-10`과 일치하지 않습니다.
- `domain`은 모든 `urlId`와 `url` 내부의 `from`을 찾아/바꾸고, 각 댓글의 `domain`을 업데이트합니다. 두 값 모두 `old.example.com`과 같은 도메인이어야 합니다.

댓글, 페이지, 페이지 반응, 투표, 알림 및 구독 모두 이동합니다. `to`에 이미 페이지가 존재하면 두 페이지가 병합됩니다.

마이그레이션은 배치 작업으로 실행됩니다. 응답에 `batchJobId`가 포함됩니다. 진행 상황을 확인하려면 [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id)를 폴링하십시오. 작업이 완료되면 마이그레이션당 결과가 제공되며, 90일 이내에 [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post)로 전달할 수 있는 `undoId`가 있습니다.

요청은 큐에 넣기 전에 검증됩니다. 마이그레이션 중 하나라도 유효하지 않으면 전체 요청이 `422`와 함께 실패하고 아무 작업도 실행되지 않습니다.

마이그레이션을 큐에 넣는 데 비용이 들지 않습니다. 작업이 완료되면 마이그레이션된 `1000`개의 댓글당 `1` 크레딧이 차감되며, 최소 `1` 크레딧이 부과됩니다.

[inline-code-attrs-start title = '페이지 마이그레이션 cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = '페이지 마이그레이션 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = '페이지 마이그레이션 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

#### 참고

- 한 테넌트에 대한 작업은 한 번에 하나씩, 오래된 것이 먼저 실행됩니다. 두 번째 마이그레이션은 첫 번째가 완료될 때까지 기다립니다.
- `PATCH /api/v1/pages/:id`를 새 `urlId`와 함께 사용하면 페이지만 변경되고 댓글은 변경되지 않습니다. 이 라우트를 사용하여 페이지의 댓글을 이동하십시오.

---