---
批次取得租戶的使用者資訊。根據 userIds，回傳來自 User / SSOUser 的顯示資訊。此功能被評論小工具使用，以在使用者透過在場事件剛出現時豐富其資訊。沒有頁面上下文：隱私權會一致地被強制執行（私人檔案會被遮蔽）。

## Parameters

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| ids | string | 是 |  |

## Response

回傳：[`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Example

[inline-code-attrs-start title = 'getUsersInfo 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchUsersInfo(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const ids: string = "user_001,user_002";
  const response: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
  console.log(response);
}
fetchUsersInfo();
[inline-code-end]

---