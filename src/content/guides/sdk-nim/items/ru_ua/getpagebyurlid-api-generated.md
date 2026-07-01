## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## Ответ

Returns: [`Option[GetPageByURLIdAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_page_by_urlid_api_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример getPageByURLId'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (pageOpt, httpRes) = client.getPageByURLId(tenantId = "my-tenant-123", urlId = "news/article-title")
if pageOpt.isSome:
  let page = pageOpt.get()
  echo page
[inline-code-end]