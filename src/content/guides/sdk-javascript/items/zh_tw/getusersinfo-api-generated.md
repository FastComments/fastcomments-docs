為租戶批次取得使用者資訊。給定 userIds，回傳來自 User / SSOUser 的顯示資訊。
由留言小工具使用，用以豐富剛透過 presence 事件出現的使用者資料。
沒有頁面上下文：隱私會統一強制執行（私人檔案會被遮蔽）。

## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| ids | string | 是 |  |

## 回應

回傳：[`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## 範例

[inline-code-attrs-start title = 'getUsersInfo 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_78f9';
const ids: string = 'user_10234,user_10235,user_10236';
const usersInfo: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
// getUsersInfo 只需要 tenantId 和 ids；可選參數在此不適用。
[inline-code-end]