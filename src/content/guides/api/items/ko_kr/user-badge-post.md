이 엔드포인트를 통해 새로운 사용자 배지 할당을 생성할 수 있습니다.

요청 예제:

[inline-code-attrs-start title = 'POST 요청 예제'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X POST "https://fastcomments.com/api/v1/user-badges?tenantId=demo&API_KEY=DEMO_API_SECRET" \
-H "Content-Type: application/json" \
-d '{
  "userId": "user456",
  "badgeId": "badgeDef789",
  "displayedOnComments": true
}'
[inline-code-end]

요청 본문에는 다음 매개변수가 포함되어야 합니다:

- `userId` (required) - 배지를 할당할 사용자의 ID
- `badgeId` (required) - 할당할 배지의 ID
- `displayedOnComments` (optional) - 배지가 사용자의 댓글에 표시될지 여부 (기본값: true)

중요 참고사항:
1. 배지는 테넌트의 배지 카탈로그에 존재하고 활성화되어 있어야 합니다
2. 배지는 귀하의 테넌트에 속해 있거나 귀하의 사이트에 댓글을 남긴 사용자에게만 할당할 수 있습니다

응답 예제:

[inline-code-attrs-start title = '응답'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "userBadge": {
    "id": "badge123",
    "userId": "user456",
    "badgeId": "badgeDef789",
    "fromTenantId": "tenant001",
    "createdAt": 1650532511000,
    "receivedAt": 1650532511000,
    "type": 14,
    "name": "Special Contributor",
    "description": "Awarded to special contributors to our community",
    "displayLabel": "Special",
    "backgroundColor": "#4a5568",
    "textColor": "#ffffff",
    "displayedOnComments": true,
    "order": 1
  }
}
[inline-code-end]

가능한 오류 응답:

[inline-code-attrs-start title = '오류: 테넌트 ID 누락'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = '오류: 사용자 ID 누락'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-user-id",
  "reason": "User ID (body param: userId) is required."
}
[inline-code-end]

[inline-code-attrs-start title = '오류: 배지 ID 누락'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-badge-id",
  "reason": "Badge ID (body param: badgeId) is required."
}
[inline-code-end]

[inline-code-attrs-start title = '오류: 배지 없음'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "badge-not-found",
  "reason": "The badge badgeDef789 was not found or is not enabled."
}
[inline-code-end]

[inline-code-attrs-start title = '오류: 권한 없는 사용자'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "unauthorized-user",
  "reason": "You can only add badges to users who belong to your tenant or have commented on your site."
}
[inline-code-end]