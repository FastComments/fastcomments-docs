## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## Response

Returns: [`Option[DeletePageAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_page_api_response.nim)

## Example

[inline-code-attrs-start title = 'deletePage Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deletePage(tenantId = "my-tenant-123", id = "news/article-title")
if response.isSome:
  let deleted = response.get()
  echo deleted
else:
  echo "Deletion failed or no response body"
[inline-code-end]
