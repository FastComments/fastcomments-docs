目前線上檢視頁面的使用者：指其 WebSocket 連線目前已訂閱該頁面的使用者。  
返回 anonCount + totalCount（全房間的訂閱者，包括我們未列舉的匿名檢視者）。

## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| afterName | string | 否 |  |
| afterUserId | string | 否 |  |

## 回應

返回：[`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOnlineResponse.ts)

## 範例

[inline-code-attrs-start title = 'getOnlineUsers 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchOnlineUsers() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "article-9876";
  const afterName: string | undefined = "john_doe";
  const afterUserId: string | undefined = "user_456";

  const onlineUsers: PageUsersOnlineResponse = await getOnlineUsers(
    tenantId,
    urlId,
    afterName,
    afterUserId
  );

  console.log(onlineUsers);
}

fetchOnlineUsers();
[inline-code-end]

---