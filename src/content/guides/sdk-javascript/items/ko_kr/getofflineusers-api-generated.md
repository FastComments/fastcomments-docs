Past commenters on the page who are NOT currently online. Sorted by displayName.  
페이지에 과거에 댓글을 남겼지만 현재 온라인이 아닌 댓글 작성자들. displayName 순으로 정렬됩니다.

Use this after exhausting /users/online to render a "Members" section.  
/users/online을 모두 사용한 후에 "Members" 섹션을 렌더링하기 위해 사용합니다.

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
commenterName에 대한 커서 페이지네이션: 서버가 부분 {tenantId, urlId, commenterName} 인덱스를 afterName 이후로 $gt를 사용해 이동하며, $skip 비용이 없습니다.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Response

Returns: [`GetOfflineUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOfflineUsersResponse.ts)

## Example

[inline-code-attrs-start title = 'getOfflineUsers 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchOfflineUsers(): Promise<void> {
    const tenantId: string = "tenant_12345";
    const urlId: string = "thread_9876";
    const afterName: string = "Jane Smith";
    const afterUserId: string = "user_7f9b3c";

    const offlineUsers: GetOfflineUsersResponse = await getOfflineUsers(
        tenantId,
        urlId,
        afterName,
        afterUserId
    );

    console.log(offlineUsers);
}
[inline-code-end]

---