[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators/:id/send-invite'; creditsCost = 10; api-resource-header-end]

이 경로는 단일 `Moderator`를 초대할 수 있는 기능을 제공합니다.

다음 제한 사항이 `Moderator`에게 초대 이메일을 보내는 데 적용됩니다:
- `Moderator`는 이미 존재해야 합니다.
- `fromName`은 `100 characters`보다 길 수 없습니다.

**참고:**
- 제공된 이메일을 가진 사용자가 이미 존재하면, 해당 사용자는 테넌트의 댓글을 관리하도록 초대됩니다.
- 제공된 이메일을 가진 사용자가 **존재하지 않는 경우**, 초대 링크는 계정 생성 절차로 안내합니다.
- 초대는 `30 days` 후에 만료됩니다.

이메일만 알고 있는 사용자에 대해 `Moderator`를 생성할 수 있습니다:

[inline-code-attrs-start title = 'Moderator 초대 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators/xyz/send-invite?tenantId=demo&API_KEY=DEMO_API_SECRET&fromName=Bob' \
  --header 'Content-Type: application/json'
[inline-code-end]

이것은 다음과 같은 이메일을 보냅니다: `Bob at TenantName is inviting you to be a moderator...`

[inline-code-attrs-start title = 'Moderator 초대 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteQueryParams {
    tenantId: string
    API_KEY: string
    /** 사용자에게 전송되는 이메일은 이 이름에서 보낸 것처럼 표시됩니다. **/
    fromName: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Moderator 초대 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found | 'from-name-required' | 'from-name-invalid'
    /** 실패 시 포함됩니다. **/
    reason?: string
}
[inline-code-end]

---