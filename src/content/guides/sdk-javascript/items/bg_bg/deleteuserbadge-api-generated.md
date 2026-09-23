## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Отговор

Returns: [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptySuccessResponse.ts)

## Пример

[inline-code-attrs-start title = 'deleteUserBadge Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const badgeId: string = "moderator-badge";
  const result: APIEmptySuccessResponse = await deleteUserBadge(tenantId, badgeId);
  console.log(result);
})();
[inline-code-end]

---