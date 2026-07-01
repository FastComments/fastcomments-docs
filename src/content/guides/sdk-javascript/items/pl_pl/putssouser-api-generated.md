## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| id | string | Tak |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Tak |  |
| updateComments | boolean | Nie |  |

## Odpowiedź

Zwraca: [`PutSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutSSOUserAPIResponse.ts)

## Przykład

[inline-code-attrs-start title = 'putSSOUser Przykład'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_abc123";
  const userId: string = "user_456def";

  const updateData: UpdateAPISSOUserData = {
    email: "jane.doe@example.com",
    displayName: "Jane Doe",
    isActive: true,
  };

  const response: PutSSOUserAPIResponse = await putSSOUser(tenantId, userId, updateData, true);
})();
[inline-code-end]