## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tag | string | Hayır |  |
| tenantId | string | Evet |  |
| updateHashTagBody | UpdateHashTagBody | Hayır |  |

## Yanıt

Döndürür: [`Option[PatchHashTag_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_patch_hash_tag200response.nim)

## Örnek

[inline-code-attrs-start title = 'patchHashTag Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.patchHashTag(tag = "politics", tenantId = "my-tenant-123", updateHashTagBody = UpdateHashTagBody())

if response.isSome:
  let updated = response.get()
  echo "Hashtag updated successfully"
else:
  echo "Failed to update hashtag, status:", httpResponse.status
[inline-code-end]

---