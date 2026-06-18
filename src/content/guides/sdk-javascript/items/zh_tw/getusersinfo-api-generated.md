租戶的批量用戶資訊。給定 userIds，從 User / SSOUser 返回顯示資訊。
由評論 widget 使用，用於豐富剛透過 presence 事件出現的使用者。
無頁面上下文：隱私統一強制執行（私人檔案會被遮蔽）。

## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| ids | string | 是 |  |

## 回應

回傳: [`GetUsersInfo200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfo200Response.ts)

## 範例

[inline-code-attrs-start title = 'getUsersInfo 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-007';
const userIdsList: string[] = ['user_12a', 'user_34b', 'user_56c'];
const separator: string | undefined = undefined; // 可選；若為 undefined 則預設為逗號
const ids: string = userIdsList.join(separator ?? ',');
const usersInfo: GetUsersInfo200Response = await getUsersInfo(tenantId, ids);
[inline-code-end]

---