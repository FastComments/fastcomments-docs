## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| tag | string | No |  |
| updateHashTagBody | UpdateHashTagBody | No |  |

## Отговор

Returns: [`Option[UpdateHashTagResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_hash_tag_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за patchHashTag'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateBody = UpdateHashTagBody()
let (optResp, httpResp) = client.patchHashTag(
  tenantId = "my-tenant-123",
  tag = "news",
  updateHashTagBody = updateBody
)
if optResp.isSome:
  let resp = optResp.get()
  echo resp
else:
  echo "No response"
[inline-code-end]