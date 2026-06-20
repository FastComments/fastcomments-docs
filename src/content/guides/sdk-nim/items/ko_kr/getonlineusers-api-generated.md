현재 페이지에 온라인 상태인 뷰어: 웹소켓 세션이 현재 해당 페이지에 구독되어 있는 사용자들입니다.
익명 뷰어를 열거하지 않는 방 전체 구독자를 포함하여 anonCount + totalCount를 반환합니다.

## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| afterName | string | 아니요 |  |
| afterUserId | string | 아니요 |  |

## 응답

반환: [`Option[PageUsersOnlineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_online_response.nim)

## 예제

[inline-code-attrs-start title = 'getOnlineUsers 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getOnlineUsers(tenantId = "my-tenant-123", urlId = "news/politics/top-story", afterName = "", afterUserId = "")
if response.isSome:
  let page = response.get()
  echo "Received online users page:"
  echo page
else:
  echo "No online users returned. HTTP status: ", httpResponse.statusCode
[inline-code-end]

---