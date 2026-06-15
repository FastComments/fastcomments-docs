## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 回應

回傳: [`UpdateUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserBadge200Response.ts)

## 範例

[inline-code-attrs-start title = 'deleteUserBadge 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_8b3f2c7a";
const badgeIdOptional: string | undefined = Math.random() > 0.5 ? "badge_4f9a21" : undefined;
const id: string = badgeIdOptional ?? "badge_backup_01";
const result: UpdateUserBadge200Response = await deleteUserBadge(tenantId, id);
[inline-code-end]

---