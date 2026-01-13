## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createHashTagBody | CreateHashTagBody | No |  |

## Response

Returns: [`Option[AddHashTag_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_hash_tag200response.nim)

## Example

[inline-code-attrs-start title = 'addHashTag Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let createBody = CreateHashTagBody(
  name = "sports",
  description = "Articles and discussions about sports",
  aliases = @["sport", "athletics"],
  isActive = true
)

let (response, httpResponse) = client.addHashTag(tenantId = "my-tenant-123", createHashTagBody = createBody)

if response.isSome:
  let added = response.get()
  echo "HashTag added successfully"
else:
  echo "Failed to add HashTag"
[inline-code-end]

---