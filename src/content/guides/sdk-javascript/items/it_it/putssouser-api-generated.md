## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Sì |  |
| updateComments | boolean | No |  |

## Risposta

Restituisce: [`PutSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutSSOUserAPIResponse.ts)

## Esempio

[inline-code-attrs-start title = 'putSSOUser Esempio'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---