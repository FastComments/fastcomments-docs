[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

이 API는 사용자가 볼 수 있도록 댓글을 가져오는 데 사용됩니다. 예를 들어, 승인되지 않았거나 스팸인 댓글을 자동으로 필터링합니다.

### Pagination

페이지네이션은 성능 요구 사항 및 사용 사례에 따라 두 가지 방법 중 하나로 수행할 수 있습니다:

1. **Fastest: Precalculated Pagination**:
   1. FastComments는 사전 구축된 위젯과 클라이언트를 사용할 때 이렇게 동작합니다.
   2. "next"를 클릭하면 페이지 수가 단순히 증가합니다.
   3. 이를 키-값 저장소에서 가져오는 것으로 생각할 수 있습니다.
   4. 이 경우 `page` 매개변수를 `0`부터 시작하도록 정의하고 정렬 방향을 `direction`으로 지정하면 됩니다.
   5. 페이지 크기는 커스터마이징 규칙을 통해 맞춤 설정할 수 있습니다.
2. **Most Flexible: Flexible Pagination**:
   1. 이 방법을 사용하면 사용자 정의 `limit` 및 `skip` 매개변수를 정의할 수 있습니다. `page`는 전달하지 마세요.
   2. `direction` 정렬도 지원됩니다.
   3. `limit`은 `skip`이 적용된 후 반환할 총 개수입니다.
      - 예: `page size = 100`이고 `page = 2`인 경우 `skip = 200, limit = 100`으로 설정합니다.
   4. 자식 댓글도 페이지네이션에 포함됩니다. `asTree` 옵션을 사용하면 이를 우회할 수 있습니다.
      - `limitChildren` 및 `skipChildren`을 사용하여 자식 댓글을 페이지네이션할 수 있습니다.
      - `maxTreeDepth`를 사용하여 반환되는 스레드의 깊이를 제한할 수 있습니다.

### Threads

1. `Precalculated Pagination`을 사용할 때, 댓글은 *page*별로 그룹화되며 스레드 내 댓글이 전체 페이지에 영향을 줍니다.
   1. 이 경우 클라이언트에서 `parentId`를 기준으로 스레드를 결정할 수 있습니다.
   2. 예를 들어, 최상위 댓글이 하나이고 29개의 답글이 있는 페이지에서 API에 `page=0`을 설정하면 최상위 댓글 하나와 29개의 자식 댓글만 반환됩니다.
2. `Flexible Pagination`을 사용할 때 `parentId` 매개변수를 정의할 수 있습니다.
   1. 이를 null로 설정하면 최상위 댓글만 가져옵니다.
   2. 그런 다음 스레드를 보려면 API를 다시 호출하고 `parentId`를 전달합니다.
   3. 일반적인 해결책은 최상위 댓글에 대해 API 호출을 한 뒤, 각 댓글의 자식 댓글을 가져오기 위해 병렬 API 호출을 수행하는 것입니다.
3. __NEW 2023년 2월부터!__ `&asTree=true`를 사용하여 트리 형태로 가져옵니다.
   1. 이를 `Flexible Pagination as a Tree`로 생각할 수 있습니다.
   2. 페이지네이션에는 최상위 댓글만 포함됩니다.
   3. 트리를 루트에서 시작하려면 `parentId=null`로 설정합니다(`parentId`를 반드시 설정해야 합니다).
   4. 페이지네이션을 위해 `skip` 및 `limit`을 설정합니다.
   5. `asTree`를 `true`로 설정합니다.
   6. 이 시나리오에서는 백엔드가 훨씬 더 많은 작업을 수행해야 하므로 크레딧 비용이 `2x` 증가합니다.
   7. 원하는 대로 `maxTreeDepth`, `limitChildren`, `skipChildren`을 설정합니다.

### Trees Explained

`asTree`를 사용할 때 페이지네이션을 이해하기 어려울 수 있습니다. 다음은 유용한 그래픽입니다:

<div class="screenshot white-bg">
    <div class="title">트리 페이지네이션 다이어그램</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="트리 페이지네이션 다이어그램" />
</div>

### Fetching Comments in The Context of a User

`/comments` API는 두 가지 컨텍스트에서 사용될 수 있으며, 각각 다른 사용 사례에 적용됩니다:

- 자신의 클라이언트를 구축하기 위해 정렬되고 태그된 정보를 포함한 댓글을 반환합니다.
  - 이 경우 `contextUserId` 쿼리 매개변수를 정의합니다.
- 맞춤형 통합을 위해 백엔드에서 댓글을 가져옵니다.
  - 플랫폼은 `contextUserId` 없이도 기본적으로 이 방식을 사용합니다.

[inline-code-attrs-start title = '댓글 사전 계산 페이지네이션'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = '댓글 유연한 페이지네이션'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = '사용자 컨텍스트에서 댓글 유연한 페이지네이션'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = '사용자 컨텍스트에서 최상위 댓글만 위한 유연한 페이지네이션'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Get Comments as a Tree

트리를 반환하도록 요청할 수 있으며, 페이지네이션은 최상위 댓글만 계산됩니다.

[inline-code-attrs-start title = '사용자 컨텍스트에서 트리 형태 댓글'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

최상위 댓글과 즉시 자식만 가져오고 싶나요? 다음과 같이 할 수 있습니다:

[inline-code-attrs-start title = '최대 깊이 제한 트리 형태 댓글'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

하지만 UI에서는 각 댓글에 “답글 보기” 버튼을 표시할지 여부를 알아야 할 수도 있습니다. 트리 형태로 댓글을 가져올 때는 해당 경우에 `hasChildren` 속성이 댓글에 태그됩니다.

### Get Comments as a Tree, Searching by Hash Tag

해시태그로 검색할 수 있으며, 전체 테넌트(특정 페이지나 `urlId`에 제한되지 않음)에서 검색합니다.

이 예시에서는 `urlId`를 생략하고 여러 해시태그로 검색합니다. API는 요청된 모든 해시태그를 포함하는 댓글만 반환합니다.

[inline-code-attrs-start title = '사용자 컨텍스트에서 해시태그별 트리 형태 댓글'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### All Request Params

[inline-code-attrs-start title = '댓글 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 댓글이 연결된 urlId(페이지 URL 또는 기사 ID) **/
    urlId?: string
    /** 이 사용자가 반환받는 댓글 수를 제한합니다. **/
    userId?: string
    /** 해시태그로 검색할 때 사용합니다. 여러 해시태그의 교집합을 찾으려면 &hashTag=a&hashTag=b와 같이 지정합니다. **/
    hashTag?: string
    /** 정렬 방향. 기본값은 MR(가장 관련성 높음)이며, 다른 옵션으로 OF(오래된 순)와 NF(최신 순)가 있습니다. **/
    direction?: 'MR' | 'OF' | 'NF'
    /** 사전 계산 페이지네이션: 가져올 페이지 번호이며, 0부터 시작합니다. 모든 댓글(최대 250개)을 가져오려면 -1을 전달합니다. **/
    page?: number
    /** 유연한 페이지네이션: 반환할 댓글 수를 지정합니다. **/
    limit?: number
    /** 유연한 페이지네이션: 각 부모에 대해 반환할 자식 댓글 수를 지정합니다. **/
    limitChildren?: number
    /** 유연한 페이지네이션: 건너뛸 댓글 수를 지정합니다. **/
    skip?: number
    /** 유연한 페이지네이션: 각 부모에 대해 건너뛸 자식 댓글 수를 지정합니다. **/
    skipChildren?: number
    /** 차단 및 신고된 댓글을 판단하기 위해 사용됩니다. **/
    contextUserId?: string
    /** 차단 및 신고된 댓글을 판단하기 위해 사용됩니다. **/
    anonUserId?: string
    /** 자식 댓글을 가져오기 위해 사용됩니다. **/
    parentId?: string
    /** 트리 형태로 가져오기 위해 사용됩니다. **/
    asTree?: boolean
    /** 트리 깊이를 얼마나 반환할지 지정합니다. 0은 자식이 없고, 1은 즉시 자식만 반환합니다. **/
    maxTreeDepth?: number
}
[inline-code-end]

### The Response

[inline-code-attrs-start title = '댓글 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** 실패 시 포함됩니다. **/
    reason?: string
    /** 댓글 목록! **/
    comments: Comment[]
}
[inline-code-end]

### Helpful Tips

#### URL ID

`Comment` API를 `urlId` 매개변수와 함께 사용하는 것이 좋습니다. 먼저 `Pages` API를 호출하면 사용 가능한 `urlId` 값이 어떻게 생겼는지 확인할 수 있습니다.

#### Anonymous Actions

익명 댓글을 달 때는 댓글을 가져올 때와 신고·차단을 수행할 때 `anonUserId`를 전달하는 것이 좋습니다.

(!) 많은 앱 스토어에서 이는 필수이며, 사용자는 로그인하지 않아도 볼 수 있는 사용자 생성 콘텐츠를 신고할 수 있어야 합니다. 이를 수행하지 않으면 해당 스토어에서 앱이 삭제될 수 있습니다.

#### Comments Not Being Returned

댓글이 승인되었고 스팸이 아닌지 확인하십시오.

---