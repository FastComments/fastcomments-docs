取得租戶的批次使用者資訊。根據 userIds，回傳 User / SSOUser 的顯示資訊。此功能由評論小工具使用，以在使用者透過 presence 事件剛出現時豐富其資訊。無頁面上下文：隱私權一致執行（私密個人檔案會被遮蔽）。

## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## 回應

返回: [`GetUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfoResponse.ts)

## 範例

[inline-code-attrs-start title = 'getUsersInfo 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const ids: string = "user-1001,user-1002";

const usersInfo: GetUsersInfoResponse = await getUsersInfo(tenantId, ids);

// 回應中的可選欄位可能為 undefined
const firstUser: PageUserEntry | undefined = usersInfo?.users?.[0];
[inline-code-end]

---