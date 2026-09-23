## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateUserBadgeParams | UpdateUserBadgeParams | Yes |  |

## 回應

返回：[`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptySuccessResponse.ts)

## 範例

[inline-code-attrs-start title = 'updateUserBadge 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-9f8b7c6d";
const id: string = "user-4a3b2c1d";

const updateParams: UpdateUserBadgeParams = {
  badgeId: "badge-premium",
  // 可選欄位
  expiresAt: new Date("2025-12-31T23:59:59Z")
};

const result: APIEmptySuccessResponse = await updateUserBadge(tenantId, id, updateParams);
[inline-code-end]

---