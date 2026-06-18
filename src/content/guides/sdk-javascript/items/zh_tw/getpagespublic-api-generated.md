---
列出租戶的頁面。供 FChat 桌面客戶端用來填充其房間列表。
要求每個頁面的解析後自訂設定中 `enableFChat` 必須為 true。
需要 SSO 的頁面會根據請求使用者的群組權限進行篩選。

## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| cursor | string | 否 |  |
| limit | number | 否 |  |
| q | string | 否 |  |
| sortBy | PagesSortBy | 否 |  |
| hasComments | boolean | 否 |  |

## 回應

回傳: [`GetPagesPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesPublic200Response.ts)

## 範例

[inline-code-attrs-start title = 'getPagesPublic 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3b2c';
const cursor: string = 'eyJwYWdlIjoiMTIwIn0';
const limit: number = 25;
const q: string = 'homepage hero';
const hasComments: boolean = true;

const response: GetPagesPublic200Response = await getPagesPublic(
  tenantId,
  cursor,
  limit,
  q,
  undefined,
  hasComments
);
[inline-code-end]

---