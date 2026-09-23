Past commenters on the page who are NOT currently online. Sorted by displayName.  
현재 온라인이 아닌 페이지의 이전 댓글 작성자들. displayName 기준으로 정렬됩니다.

Use this after exhausting /users/online to render a "Members" section.  
`/users/online`을 모두 사용한 후에 "Members" 섹션을 렌더링하기 위해 사용합니다.

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName}  
index from afterName forward via $gt, no $skip cost.  
commenterName에 대한 커서 페이지네이션: 서버가 부분 {tenantId, urlId, commenterName}을 순회합니다. afterName 이후를 $gt를 통해 앞으로 이동하며, $skip 비용이 없습니다.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Response

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## Example

[inline-code-attrs-start title = 'getOfflineUsers 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchOfflineUsers(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const urlId: string = "page_9876";
  const afterName: string = "John Doe";
  const afterUserId: string = "user_abc123";

  const offlineResponse: PageUsersOfflineResponse = await getOfflineUsers(
    tenantId,
    urlId,
    afterName,
    afterUserId
  );

  console.log(offlineResponse);
}
[inline-code-end]