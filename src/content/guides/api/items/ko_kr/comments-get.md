[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

이 API는 사용자에게 표시할 댓글을 가져오는 데 사용됩니다. 예를 들어, 승인되지 않았거나 스팸으로 표시된 댓글을 자동으로 필터링합니다.

### 페이지네이션

페이지네이션은 성능 요구 사항과 사용 사례에 따라 두 가지 방법 중 하나로 수행할 수 있습니다:

1. 가장 빠름: **사전 계산된 페이지네이션(Precalculated Pagination)**:
   1. FastComments의 사전 빌드된 위젯과 클라이언트를 사용할 때 동작하는 방식입니다.
   2. "다음"을 클릭하면 단순히 페이지 수가 증가합니다.
   3. 키-값 저장소에서 가져오는 것으로 생각할 수 있습니다.
   4. 이 방식에서는 `0`부터 시작하는 `page` 매개변수와 정렬 방향으로 `direction`을 정의하기만 하면 됩니다.
   5. 페이지 크기는 커스터마이제이션 규칙을 통해 사용자 정의할 수 있습니다.
2. 가장 유연함: **유연한 페이지네이션(Flexible Pagination)**:
   1. 이 방식에서는 사용자 정의 `limit` 및 `skip` 매개변수를 정의할 수 있습니다. `page`를 전달하지 마세요.
   2. 정렬 `direction`도 지원됩니다.
   3. `limit`은 `skip`이 적용된 후 반환할 총 개수입니다.
      - 예: `page size = 100`이고 `page = 2`일 때 `skip = 200, limit = 100`으로 설정합니다.
   4. 자식 댓글도 페이지네이션에 포함됩니다. `asTree` 옵션을 사용하여 이를 우회할 수 있습니다.
      - `limitChildren` 및 `skipChildren`을 통해 자식 댓글을 페이지네이션할 수 있습니다.
      - `maxTreeDepth`를 통해 반환되는 스레드의 깊이를 제한할 수 있습니다.

### 스레드

1. `Precalculated Pagination`을 사용할 때, 댓글은 *페이지*별로 그룹화되며 스레드의 댓글이 전체 페이지에 영향을 줍니다.
   1. 이 방식에서는 클라이언트에서 `parentId`를 기반으로 스레드를 결정할 수 있습니다.
   2. 예를 들어, 최상위 댓글 1개와 답글 29개가 있는 페이지에서 API에 `page=0`을 설정하면 최상위 댓글과 29개의 자식 댓글만 받게 됩니다.
2. `Flexible Pagination`을 사용할 때는 `parentId` 매개변수를 정의할 수 있습니다.
   1. 최상위 댓글만 가져오려면 이 값을 null로 설정하세요.
   2. 그런 다음 스레드를 보려면 API를 다시 호출하고 `parentId`를 전달합니다.
   3. 일반적인 해결책은 최상위 댓글에 대해 API를 호출한 다음, 각 댓글의 자식 댓글을 가져오기 위해 병렬로 API 호출을 수행하는 것입니다.
3. __2023년 2월부터 새로 추가됨!__ `&asTree=true`를 사용하여 트리 형태로 가져올 수 있습니다.
   1. 이를 `트리 형태의 유연한 페이지네이션`이라고 생각할 수 있습니다.
   2. 페이지네이션에는 최상위 댓글만 계산됩니다.
   3. 트리를 루트에서 시작하려면 `parentId=null`로 설정하세요 (`parentId`를 반드시 설정해야 합니다).
   4. 페이지네이션을 위해 `skip`과 `limit`을 설정합니다.
   5. `asTree`를 `true`로 설정합니다.
   6. 이 시나리오에서는 백엔드가 훨씬 더 많은 작업을 수행해야 하므로 크레딧 비용이 `2배` 증가합니다.
   7. `maxTreeDepth`, `limitChildren`, `skipChildren`을 원하는 대로 설정합니다.

### 트리 설명

`asTree`를 사용할 때는 페이지네이션을 이해하기 어려울 수 있습니다. 다음은 유용한 그림입니다:

<div class="screenshot white-bg">
    <div class="title">트리 페이지네이션 다이어그램</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="트리 페이지네이션 다이어그램" />
</div>

### 사용자 컨텍스트에서 댓글 가져오기

`/comments` API는 서로 다른 사용 사례를 위해 두 가지 컨텍스트에서 사용할 수 있습니다:

- 자체 클라이언트를 구축하기 위한 정보로 정렬되고 태그가 지정된 댓글을 반환하는 경우.
  - 이 경우 `contextUserId` 쿼리 매개변수를 정의합니다.
- 커스텀 통합을 위해 백엔드에서 댓글을 가져오는 경우.
  - `contextUserId`가 없으면 플랫폼은 기본적으로 이 컨텍스트를 사용합니다.

[inline-code-attrs-start title = '댓글 사전 계산된 페이지네이션'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = '사용자 컨텍스트에서 최상위 댓글만 유연한 페이지네이션으로 가져오기'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### 트리 형태로 댓글 가져오기

페이지네이션이 최상위 댓글만 계산하는 트리 형태로 댓글을 반환받을 수 있습니다.

[inline-code-attrs-start title = '사용자 컨텍스트에서 트리 형태 댓글'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

최상위 댓글과 바로 아래 자식 댓글만 가져오고 싶으신가요? 다음은 한 가지 방법입니다:

[inline-code-attrs-start title = '최대 깊이가 지정된 트리 형태 댓글'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

하지만 UI에서 각 댓글에 "답글 보기" 버튼을 표시할지 여부를 알아야 할 수 있습니다. 트리 형태로 댓글을 가져올 때 해당되는 경우 댓글에 `hasChildren` 속성이 태그됩니다.

### 해시태그로 검색하여 트리 형태로 댓글 가져오기

API를 사용하여 전체 테넌트(한 페이지 또는 `urlId`에 국한되지 않음)에서 해시태그로 검색할 수 있습니다.

이 예제에서는 `urlId`를 생략하고 여러 해시태그로 검색합니다. API는 요청된 모든 해시태그가 포함된 댓글만 반환합니다.

[inline-code-attrs-start title = '사용자 컨텍스트에서 해시태그로 트리 형태 댓글'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### 모든 요청 매개변수

[inline-code-attrs-start title = '댓글 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 댓글이 연관된 urlId (페이지 URL 또는 아티클 ID). **/
    urlId?: string
    /** 이 사용자의 댓글로 결과를 제한합니다. **/
    userId?: string
    /** 해시태그로 검색하는 데 사용합니다. 여러 해시태그의 교집합으로 좁히려면 &hashTag=a&hashTag=b 형식으로 사용하세요. **/
    hashTag?: string
    /** 정렬 방향. 기본값은 MR (가장 관련성 높음)입니다. 다른 옵션은 OF (오래된 순)와 NF (최신 순)입니다. **/
    direction?: 'MR' | 'OF' | 'NF'
    /** 사전 계산된 페이지네이션: 가져올 페이지, 0부터 시작. 모든 댓글(최대 250개)을 가져오려면 -1을 전달하세요. **/
    page?: number
    /** 유연한 페이지네이션: 댓글을 몇 개 반환할까요? **/
    limit?: number
    /** 유연한 페이지네이션: 각 부모에 대해 자식 댓글을 몇 개 반환할까요? **/
    limitChildren?: number
    /** 유연한 페이지네이션: 댓글을 몇 개 건너뛸까요? **/
    skip?: number
    /** 유연한 페이지네이션: 각 부모에 대해 자식 댓글을 몇 개 건너뛸까요? **/
    skipChildren?: number
    /** 차단 및 신고된 댓글을 판별하기 위한 값. **/
    contextUserId?: string
    /** 차단 및 신고된 댓글을 판별하기 위한 값. **/
    anonUserId?: string
    /** 자식 댓글을 가져오기 위한 값. **/
    parentId?: string
    /** 트리 형태로 가져오기 위한 값. **/
    asTree?: boolean
    /** 트리의 어디까지 데이터를 반환할까요? 0은 자식을 반환하지 않습니다. 1은 바로 아래 자식을 반환하는 식입니다. **/
    maxTreeDepth?: number
}
[inline-code-end]

### 응답

[inline-code-attrs-start title = '댓글 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** 실패 시 포함됩니다. **/
    reason?: string
    /** 댓글입니다! **/
    comments: Comment[]
}
[inline-code-end]

### 유용한 팁

#### URL ID

`Comment` API를 `urlId` 매개변수와 함께 사용하는 것이 좋습니다. `Pages` API를 먼저 호출하여 사용 가능한 `urlId` 값이 어떤지 확인할 수 있습니다.

#### 익명 작업

익명 댓글 작성의 경우 댓글을 가져올 때, 그리고 신고 및 차단을 수행할 때 `anonUserId`를 전달하는 것이 좋습니다.

(!) 많은 앱 스토어에서는 로그인하지 않은 사용자라도 볼 수 있는 사용자 생성 콘텐츠를 신고할 수 있어야 하므로 이 작업이 필수입니다. 이를 준수하지 않으면 앱이 해당 스토어에서 제거될 수 있습니다.

#### 댓글이 반환되지 않는 경우

댓글이 승인되었는지, 스팸이 아닌지 확인하세요.