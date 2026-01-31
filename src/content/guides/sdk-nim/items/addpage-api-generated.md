## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createAPIPageData | CreateAPIPageData | No |  |

## Response

Returns: [`Option[AddPageAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_page_api_response.nim)

## Example

[inline-code-attrs-start title = 'addPage Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let pageData = CreateAPIPageData(
  urlId = "news/market-rally",
  title = "Market Rally: Stocks Surge",
  url = "https://news.example.com/news/market-rally",
  isArchived = false,
  tags = @["finance", "markets"],
  allowComments = true
)

let (response, httpResponse) = client.addPage(tenantId = "my-tenant-123", createAPIPageData = pageData)

if response.isSome:
  let addedPage = response.get()
  discard addedPage
[inline-code-end]
