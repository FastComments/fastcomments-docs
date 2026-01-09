이 엔드포인트를 사용하면 다양한 기준으로 사용자 배지를 가져올 수 있습니다.

Example Request:

[inline-code-attrs-start title = 'GET 요청 예시'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badges?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

You can add various query parameters to filter the results:

- `userId` - 특정 사용자의 배지를 가져옵니다
- `badgeId` - 특정 배지의 인스턴스를 가져옵니다
- `type` - 배지 유형별로 필터링 (0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, etc. See UserBadge structure for full list)
- `displayedOnComments` - 배지가 댓글에 표시되는지 여부로 필터링 (true/false)
- `limit` - 반환할 배지의 최대 개수 (기본값 30, 최대 200)
- `skip` - 건너뛸 배지 수 (페이지네이션용)

Example Response:

[inline-code-attrs-start title = '응답'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "userBadges": [
    {
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
    },
    {
      "id": "badge124",
      "userId": "user456",
      "badgeId": "badgeDef790",
      "fromTenantId": "tenant001",
      "createdAt": 1650532598000,
      "receivedAt": 1650532598000,
      "type": 0,
      "threshold": 100,
      "name": "Centurion",
      "description": "Made 100 comments",
      "displayLabel": "100",
      "backgroundColor": "#2b6cb0",
      "textColor": "#ffffff",
      "displayedOnComments": true,
      "order": 2
    }
  ]
}
[inline-code-end]

Possible Error Responses:

[inline-code-attrs-start title = '오류: 누락된 테넌트 ID'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = '오류: 잘못된 limit'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "invalid-limit",
  "reason": "The limit (query param: limit) is too large (> 200)."
}
[inline-code-end]