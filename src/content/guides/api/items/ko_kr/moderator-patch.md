[api-resource-header-start name = 'Moderator'; route = 'PATCH /api/v1/moderators/:id'; creditsCost = 1; api-resource-header-end]

이 API 엔드포인트는 `id`로 `Moderator`를 업데이트할 수 있는 기능을 제공합니다.

Moderator를 업데이트할 때 다음과 같은 제한이 있습니다:

- Moderator를 업데이트할 때 다음 값들은 제공할 수 없습니다:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- `userId`가 지정된 경우, 해당 사용자가 존재해야 합니다.
- `userId`가 지정된 경우, 해당 사용자는 쿼리 매개변수에 지정된 동일한 `tenantId`에 속해야 합니다.
- 동일한 테넌트 내의 두 명의 모더레이터는 동일한 `email`로 추가할 수 없습니다.
- Moderator에 연관된 `tenantId`는 변경할 수 없습니다.

[inline-code-attrs-start title = 'Moderator PATCH cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/moderators/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Moderator PATCH 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Moderator PATCH 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface ModeratorPatchResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found';  
    /** 실패 시 포함됩니다. **/
    reason?: string
}
[inline-code-end]

---