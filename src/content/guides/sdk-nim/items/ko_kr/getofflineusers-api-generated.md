현재 온라인 상태가 아닌, 해당 페이지의 과거 댓글 작성자입니다. displayName 기준으로 정렬됩니다.
/users/online을 모두 조회한 후 "멤버" 섹션을 렌더링할 때 이 API를 사용하세요.
commenterName 기준 커서 페이지네이션: 서버는 부분 {tenantId, urlId, commenterName}
인덱스를 afterName부터 $gt로 순회하며, $skip 비용이 없습니다.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | GetOfflineUsersOptions | No |  |

## Response

반환값: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Example

[inline-code-attrs-start title = 'getOfflineUsers 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (offlineResp, httpResponse) = client.getOfflineUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  options = GetOfflineUsersOptions()
)
if offlineResp.isSome:
  let offline = offlineResp.get()
  echo offline)
[inline-code-end]