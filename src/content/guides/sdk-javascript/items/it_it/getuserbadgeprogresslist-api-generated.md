## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| userId | string | No |  |
| limit | number | No |  |
| skip | number | No |  |

## Risposta

Restituisce: [`APIGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetUserBadgeProgressListResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getUserBadgeProgressList'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant-01";
  const userId: string | undefined = "user-9e12b4";
  const limit: number | undefined = 20;
  const skip: number | undefined = 0;
  const result: APIGetUserBadgeProgressListResponse = await getUserBadgeProgressList(tenantId, userId, limit, skip);
  console.log(result);
})();
[inline-code-end]

---