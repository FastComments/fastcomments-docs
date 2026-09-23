Bulk user info for a tenant. Given userIds, return display info from User / SSOUser.  
Used by the comment widget to enrich users that just appeared via a presence event.  
No page context: privacy is enforced uniformly (private profiles are masked).

## Parameters

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| ids | string | Да |  |

## Response

Връща: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Example

[inline-code-attrs-start title = 'getUsersInfo Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchUsersInfo(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const ids: string = "user_001,user_002";
  const response: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
  console.log(response);
}
fetchUsersInfo();
[inline-code-end]

---