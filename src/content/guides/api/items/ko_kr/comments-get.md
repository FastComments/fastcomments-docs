[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

이 API는 사용자에게 표시할 댓글을 가져오는 데 사용됩니다. 예를 들어 승인되지 않았거나 스팸인 댓글은 자동으로 필터링됩니다.

### 페이지네이션

페이지네이션은 성능 요구사항과 사용 사례에 따라 두 가지 방법 중 하나로 수행할 수 있습니다:

1. 가장 빠름: **사전 계산된 페이지네이션(Precalculated Pagination)**:
   1. 이것이 FastComments가 우리 미리 제작된 위젯과 클라이언트를 사용할 때 작동하는 방식입니다.
   2. "다음"을 클릭하면 단순히 페이지 수가 증가합니다.
   3. 이를 키-값 저장소에서 검색되는 것으로 생각할 수 있습니다.
   4. 이 방식에서는 `page` 파라미터를 `0`부터 정의하고 정렬 방향을 `direction`으로 지정하면 됩니다.
   5. 페이지 크기는 커스터마이제이션 규칙을 통해 맞춤 설정할 수 있습니다.
2. 가장 유연함: **유연한 페이지네이션(Flexible Pagination)**:
   1. 이 방식에서는 사용자 정의 `limit` 및 `skip` 파라미터를 정의할 수 있습니다. `page`는 전달하지 마세요.
   2. 정렬 `direction`도 지원됩니다.
   3. `limit`은 `skip`이 적용된 후 반환할 전체 수입니다.
      - 예시: `page size = 100`이고 `page = 2`인 경우 `skip = 200, limit = 100`으로 설정합니다.
   4. 자식 댓글도 여전히 페이지네이션에 포함됩니다. `asTree` 옵션을 사용하여 이를 회피할 수 있습니다.
      - `limitChildren` 및 `skipChildren`으로 자식들을 페이지네이션할 수 있습니다.
      - 반환되는 스레드의 깊이를 `maxTreeDepth`로 제한할 수 있습니다.

### 스레드

1. `Precalculated Pagination`을 사용할 때, 댓글은 *페이지*별로 그룹화되며 스레드의 댓글이 전체 페이지에 영향을 미칩니다.
   1. 이 방식에서는 클라이언트가 `parentId` 기반으로 스레드를 결정할 수 있습니다.
   2. 예를 들어, 한 개의 최상위 댓글과 29개의 답글이 있는 페이지에서 API에 `page=0`을 설정하면 최상위 댓글 하나와 29개의 자식만 받습니다.
   3. [예시 이미지(여러 페이지를 설명).](https://blog.winricklabs.com/images/fc-pagination02.png)
2. `Flexible Pagination`을 사용할 때는 `parentId` 파라미터를 정의할 수 있습니다.
   1. 최상위 댓글만 가져오려면 이것을 null로 설정하세요.
   2. 그런 다음 스레드를 보려면 API를 다시 호출하고 `parentId`를 전달하세요.
   3. 일반적인 해결책은 최상위 댓글에 대해 API 호출을 하고 각 댓글의 자식 댓글을 가져오기 위해 병렬 API 호출을 하는 것입니다.
3. __NEW As of Feb 2023!__ `&asTree=true`를 사용하여 트리로 가져올 수 있습니다.
   1. 이를 `Flexible Pagination as a Tree`로 생각할 수 있습니다.
   2. 페이지네이션에는 최상위 댓글만 계산됩니다.
   3. 트리를 루트에서 시작하려면 `parentId=null`로 설정하세요(반드시 `parentId`를 설정해야 합니다).
   4. 페이지네이션을 위해 `skip` 및 `limit`을 설정하세요.
   5. `asTree`를 `true`로 설정하세요.
   6. 이 경우 백엔드가 더 많은 작업을 수행해야 하므로 크레딧 비용이 `2x` 증가합니다.
   7. 필요에 따라 `maxTreeDepth`, `limitChildren`, `skipChildren`를 설정하세요.

### 트리 설명

`asTree`를 사용할 때 페이지네이션을 이해하기 어려울 수 있습니다. 다음은 유용한 그래픽입니다:

<div class="screenshot white-bg">
    <div class="title">Tree Pagination Diagram</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Tree Pagination Diagram" />
</div>

### 사용자 컨텍스트에서 댓글 가져오기

`/comments` API는 두 가지 컨텍스트에서 사용될 수 있으며 서로 다른 사용 사례에 적합합니다:

- 자체 클라이언트를 빌드하기 위해 정렬되고 태그된 정보를 반환하려는 경우.
  - 이 경우 `contextUserId` 쿼리 파라미터를 정의하세요.
- 맞춤 통합을 위해 백엔드에서 댓글을 가져오는 경우.
  - 플랫폼은 `contextUserId` 없이 기본적으로 이 모드를 사용합니다.

[inline-code-attrs-start title = '댓글 사전 계산 페이지네이션'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = '댓글 유연 페이지네이션'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = '사용자 컨텍스트에서의 댓글 유연 페이지네이션'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = '최상위 댓글만을 위한 사용자 컨텍스트의 댓글 유연 페이지네이션'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### 댓글을 트리로 가져오기

댓글을 트리로 반환받을 수 있으며, 페이지네이션은 최상위 댓글만 계산합니다.

[inline-code-attrs-start title = '사용자 컨텍스트에서의 트리형 댓글'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

최상위 댓글과 바로 아래 자식들만 받고 싶나요? 다음과 같이 할 수 있습니다:

[inline-code-attrs-start title = '최대 깊이의 트리형 댓글'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

그러나 UI에서 각 댓글에 "답글 보기" 버튼을 표시할지 여부를 알아야 할 수 있습니다. 트리로 댓글을 가져올 때는 해당 댓글에 적용 가능한 경우 `hasChildren` 속성이 태그로 붙습니다.

### 해시태그로 검색하여 트리형 댓글 가져오기

API를 사용하면 테넌트 전체(한 페이지나 `urlId`에 한정되지 않음)를 대상으로 해시태그로 검색할 수 있습니다.

이 예제에서는 `urlId`를 생략하고 여러 해시태그로 검색합니다. API는 요청된 모든 해시태그를 가진 댓글만 반환합니다.

[inline-code-attrs-start title = '해시태그로 검색한 사용자 컨텍스트의 트리형 댓글'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### 모든 요청 파라미터

[inline-code-attrs-start title = '댓글 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 댓글이 연결된 urlId(페이지 URL 또는 기사 ID). **/
    urlId?: string
    /** 이 사용자가 작성한 댓글로 반환을 제한합니다. **/
    userId?: string
    /** 해시태그로 검색할 때 사용합니다. 여러 해시태그의 교집합으로 좁히려면 &hashTag=a&hashTag=b 를 사용하세요. **/
    hashTag?: string
    /** 정렬 방향. 기본값은 MR(가장 관련성 높은 순). 다른 옵션으로는 OF(오래된 것부터) 및 NF(최신순)가 있습니다. **/
    direction?: 'MR' | 'OF' | 'NF'
    /** 사전 계산 페이지네이션: 가져올 페이지로, 0부터 시작합니다. 모든 댓글을 원하면 -1을 전달하세요(최대 250). **/
    page?: number
    /** 유연한 페이지네이션: 몇 개의 댓글을 반환할지 지정합니다. **/
    limit?: number
    /** 유연한 페이지네이션: 각 부모에 대해 몇 개의 자식 댓글을 반환할지 지정합니다. **/
    limitChildren?: number
    /** 유연한 페이지네이션: 몇 개의 댓글을 건너뛸지 지정합니다. **/
    skip?: number
    /** 유연한 페이지네이션: 각 부모에 대해 몇 개의 자식 댓글을 건너뛸지 지정합니다. **/
    skipChildren?: number
    /** 차단 및 신고된 댓글을 판단하기 위해 사용됩니다. **/
    contextUserId?: string
    /** 차단 및 신고된 댓글을 판단하기 위해 사용됩니다. **/
    anonUserId?: string
    /** 자식 댓글을 가져오기 위해 사용됩니다. **/
    parentId?: string
    /** 트리로 가져오기 위해 사용됩니다. **/
    asTree?: boolean
    /** 트리에서 얼마나 깊이 들어갈지 지정합니다. 0이면 자식을 반환하지 않습니다. 1이면 바로 아래 자식을 반환합니다. **/
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
    /** 댓글들입니다! **/
    comments: Comment[]
}
[inline-code-end]

### 유용한 팁

#### URL ID

대부분의 경우 `urlId` 파라미터와 함께 `Comment` API를 사용하고 싶을 것입니다. 먼저 `Pages` API를 호출하여 사용 가능한 `urlId` 값이 어떻게 생겼는지 확인할 수 있습니다. 

#### 익명 액션

익명 댓글 기능을 위해서는 댓글을 가져올 때, 그리고 신고 및 차단을 수행할 때 `anonUserId`를 전달하는 것이 좋습니다.

(!) 많은 앱 스토어에서는 사용자가 로그인하지 않았더라도 자신이 볼 수 있는 사용자 생성 콘텐츠를 신고할 수 있어야 하기 때문에 이것이 요구됩니다. 이를 준수하지 않으면 해당 스토어에서 앱이 제거될 수 있습니다.

#### 댓글이 반환되지 않는 경우

댓글이 승인되었는지, 스팸이 아닌지 확인하세요.