---
## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Odgovor

Returns: [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptySuccessResponse.ts)

## Primjer

[inline-code-attrs-start title = 'deleteUserBadge Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const badgeId: string = "moderator-badge";
  const result: APIEmptySuccessResponse = await deleteUserBadge(tenantId, badgeId);
  console.log(result);
})();
[inline-code-end]

---