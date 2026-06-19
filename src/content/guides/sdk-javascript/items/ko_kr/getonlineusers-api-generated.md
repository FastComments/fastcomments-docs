---
현재 페이지에 온라인 상태인 뷰어: websocket 세션이 현재 해당 페이지를 구독하고 있는 사람들.
anonCount + totalCount를 반환합니다 (룸 전체 구독자, 우리가 열거하지 않는 익명 뷰어 포함).

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| afterName | string | 아니요 |  |
| afterUserId | string | 아니요 |  |

## Response

반환: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOnlineResponse.ts)

## Example

[inline-code-attrs-start title = 'getOnlineUsers 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3c2b7';
const urlId: string = 'article-2026-06-19-site-update';
const afterName: string = 'michael.hansen';
const afterUserId: string = 'user_00421';
const onlineUsers: PageUsersOnlineResponse = await getOnlineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---