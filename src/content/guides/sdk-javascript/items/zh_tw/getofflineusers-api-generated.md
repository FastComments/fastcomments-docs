頁面上目前不在線上的過往留言者。按 displayName 排序。
在用完 /users/online 之後,使用此端點來渲染「成員」區塊。
基於 commenterName 的游標分頁:伺服器透過 $gt 從 afterName 開始沿著部分索引 {tenantId, urlId, commenterName} 向前遍歷,沒有 $skip 的成本。

## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| afterName | string | 否 |  |
| afterUserId | string | 否 |  |

## 回應

回傳:[`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## 範例

[inline-code-attrs-start title = 'getOfflineUsers 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchOfflineUsers(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const urlId: string = "page_9876";
  const afterName: string = "John Doe";
  const afterUserId: string = "user_abc123";

  const offlineResponse: PageUsersOfflineResponse = await getOfflineUsers(
    tenantId,
    urlId,
    afterName,
    afterUserId
  );

  console.log(offlineResponse);
}
[inline-code-end]