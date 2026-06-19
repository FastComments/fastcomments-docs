列出租戶的頁面。供 FChat 桌面用戶端用來填充其房間列表。
需要在解析出的每個頁面的自訂設定中，`enableFChat` 為 true。
需要 SSO 的頁面會依照請求使用者的群組存取權進行過濾。

## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| cursor | string | 否 |  |
| limit | number | 否 |  |
| q | string | 否 |  |
| sortBy | PagesSortBy | 否 |  |
| hasComments | boolean | 否 |  |

## 回應

回傳：[`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## 範例

[inline-code-attrs-start title = 'getPagesPublic 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f9b2c";
const cursor: string = "cursor_0001a2b3";
const limit: number = 25;
const q: string = "product page";
const hasComments: boolean = true;
const response: GetPublicPagesResponse = await getPagesPublic(tenantId, cursor, limit, q, undefined, hasComments);
[inline-code-end]

---