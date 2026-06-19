在該頁面上過去曾發表評論但目前不在線上的使用者。依 displayName 排序。
在用盡 /users/online 後使用此項以呈現「成員」區段。
在 commenterName 上的 Cursor 分頁：伺服器會沿著部分索引 {tenantId, urlId, commenterName} 從 afterName 往前以 $gt 查詢，無需 $skip 成本。

## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| afterName | string | 否 |  |
| afterUserId | string | 否 |  |

## 回應

回傳: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## 範例

[inline-code-attrs-start title = 'getOfflineUsers 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-9f4b2a6c';
const urlId: string = 'articles/product-launch-2025';

const offlinePageFirst: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId);

const afterName: string = 'samantha.r';
const afterUserId: string = 'user_7d3a21f9';
const offlinePageNext: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---