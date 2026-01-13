## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tag | string | No |  |
| tenantId | string | Yes |  |
| updateHashTagBody | UpdateHashTagBody | No |  |

## Ответ

Возвращает: [`Option[PatchHashTag_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_patch_hash_tag200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример patchHashTag'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.patchHashTag(tag = "politics", tenantId = "my-tenant-123", updateHashTagBody = UpdateHashTagBody())

if response.isSome:
  let updated = response.get()
  echo "Hashtag updated successfully"
else:
  echo "Failed to update hashtag, status:", httpResponse.status
[inline-code-end]