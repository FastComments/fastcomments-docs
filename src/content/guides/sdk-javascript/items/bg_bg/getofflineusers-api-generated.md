Past commenters on the page who are NOT currently online. Sorted by displayName.  
Use this after exhausting /users/online to render a "Members" section.  
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName}  
index from afterName forward via $gt, no $skip cost.

## Parameters

| Име | Тип | Задължително | Описание |
|------|------|--------------|----------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| afterName | string | Не |  |
| afterUserId | string | Не |  |

## Response

Връща: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## Example

[inline-code-attrs-start title = 'Пример за getOfflineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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