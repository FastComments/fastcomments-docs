列出租戶的頁面。供 FChat 桌面客戶端填充其房間列表使用。  
需要在每個頁面的已解析自訂設定中將 `enableFChat` 設為 true。  
需要 SSO 的頁面會根據請求使用者的群組存取權限進行過濾。

## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| cursor | string | 否 |  |
| limit | number | 否 |  |
| q | string | 否 |  |
| sortBy | PagesSortBy | 否 |  |
| hasComments | boolean | 否 |  |

## 回應

返回：[`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## 範例

[inline-code-attrs-start title = 'getPagesPublic 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPublicPages() {
  const tenantId: string = "tenant_12345";
  const cursor: string = "page_5";
  const limit: number = 20;
  const query: string = "support";
  const hasComments: boolean = true;

  const response: GetPublicPagesResponse = await getPagesPublic(
    tenantId,
    cursor,
    limit,
    query,
    undefined,
    hasComments
  );

  console.log(response);
}
[inline-code-end]

---