## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | Yes |  |

## 回應

Returns: [`Option[GetPagesAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pages_api_response.nim)

## 範例

[inline-code-attrs-start title = 'getPages 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (pagesOpt, httpResp) = client.getPages(tenantId = "my-tenant-123")
if pagesOpt.isSome:
  let pages = pagesOpt.get()
  echo pages
else:
  echo "No pages returned"
echo httpResp
[inline-code-end]