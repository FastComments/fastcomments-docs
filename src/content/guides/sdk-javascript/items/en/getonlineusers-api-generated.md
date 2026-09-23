Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Response

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOnlineResponse.ts)

## Example

[inline-code-attrs-start title = 'getOnlineUsers Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchOnlineUsers() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "article-9876";
  const afterName: string | undefined = "john_doe";
  const afterUserId: string | undefined = "user_456";

  const onlineUsers: PageUsersOnlineResponse = await getOnlineUsers(
    tenantId,
    urlId,
    afterName,
    afterUserId
  );

  console.log(onlineUsers);
}

fetchOnlineUsers();
[inline-code-end]
