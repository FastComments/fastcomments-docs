## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| id | string | No |  |
| title | string | No |  |

## Response

Returns: [`Option[CreateV1PageReact]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_v1_page_react.nim)

## Example

[inline-code-attrs-start title = 'createV2PageReact Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createV2PageReact(
  tenantId = "my-tenant-123",
  urlId = "news/2026/06/fastcomments-release",
  id = "",
  title = ""
)
if response.isSome:
  let react = response.get()
  echo "Created page react: ", $react
else:
  echo "No react returned, HTTP status: ", $httpResponse.statusCode
[inline-code-end]
