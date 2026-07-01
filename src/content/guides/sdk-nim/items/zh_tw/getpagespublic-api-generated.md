列出租戶的頁面。由 FChat 桌面客戶端使用，以填充其房間列表。  
需要在每個頁面的已解析自訂設定中將 `enableFChat` 設為 true。  
需要 SSO 的頁面會根據請求使用者的群組存取權限進行篩選。

## Parameters

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| options | GetPagesPublicOptions | 否 |  |

## Response

Returns: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Example

[inline-code-attrs-start title = 'getPagesPublic 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(tenantId = "my-tenant-123", options = GetPagesPublicOptions())
if response.isSome:
  let pages = response.get()
  echo pages
[inline-code-end]