[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

이 경로는 단일 `Moderator`를 추가할 수 있는 기능을 제공합니다.

`Moderator` 생성에는 다음과 같은 제한이 있습니다:

- `name`과 `email`은 항상 제공되어야 합니다. `userId`는 선택 사항입니다.
- `Moderator`를 생성할 때 다음 값들은 제공할 수 없습니다:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- `userId`가 지정된 경우, 해당 사용자는 존재해야 합니다.
- `userId`가 지정된 경우, 해당 사용자는 쿼리 매개변수에 지정된 동일한 `tenantId`에 속해야 합니다.
- 동일한 테넌트 내에서 두 명의 모더레이터는 같은 `email`로 추가될 수 없습니다.

이메일만 알고 있는 사용자에 대해 `Moderator`를 생성할 수 있습니다:

[inline-code-attrs-start title = 'Moderator 이메일로 생성 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

또는 해당 사용자가 우리 테넌트에 속해 있는 경우, 그들의 모더레이션 통계를 추적하기 위해 `Moderator`를 생성할 수 있습니다:

[inline-code-attrs-start title = 'Moderator 테넌트 사용자로 생성 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com",
    "userId": "some-tenant-user-id"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Moderator 생성 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Moderator 생성 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found'
    /** 실패 시 포함됩니다. **/
    reason?: string
    moderator?: Moderator; // 성공 시 생성된 전체 모더레이터를 반환합니다.
}
[inline-code-end]

---