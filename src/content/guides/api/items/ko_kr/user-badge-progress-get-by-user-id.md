---
이 엔드포인트를 사용하면 사용자 ID로 사용자의 배지 진행 기록을 가져올 수 있습니다.

요청 예제:

[inline-code-attrs-start title = 'GET 요청 예제'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badge-progress/user/user456?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

응답 예제:

[inline-code-attrs-start title = '응답'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "userBadgeProgress": {
    "id": "progress123",
    "tenantId": "tenant001",
    "userId": "user456",
    "firstCommentId": "comment789",
    "firstCommentDate": 1650532511000,
    "autoTrustFactor": 0.75,
    "progress": {
      "0": 42,
      "1": 120,
      "2": 15,
      "3": 3,
      "5": 5,
      "6": 1800000,
      "8": 0,
      "7": 0
    }
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
  "reason": "The User ID (path param: userId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = '오류: 찾을 수 없음'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "not-found",
  "reason": "No badge progress found for user user456 in tenant tenant001."
}
[inline-code-end]
---