페이지의 이전 댓글 작성자(현재 온라인 상태 아님). displayName으로 정렬됩니다.
이것은 /users/online을 모두 확인한 후 'Members' 섹션을 렌더링할 때 사용합니다.
commenterName에 대한 커서 페이징: 서버는 부분 {tenantId, urlId, commenterName} 인덱스를 afterName 이후부터 $gt로 앞으로 탐색합니다. $skip 비용이 없습니다.

## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| afterName | string | 아니요 |  |
| afterUserId | string | 아니요 |  |

## 응답

반환: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## 예제

[inline-code-attrs-start title = 'getOfflineUsers 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getOfflineUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-how-to-code",
  afterName = "",
  afterUserId = ""
)

if response.isSome:
  let offlinePage = response.get()
  echo "Received offline users page"
  discard httpResponse.statusCode
[inline-code-end]

---