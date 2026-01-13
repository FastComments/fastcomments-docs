## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | Не |  |

## Отговор

Връща: [`Option[AddHashTagsBulk_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_hash_tags_bulk200response.nim)

## Пример

[inline-code-attrs-start title = 'addHashTagsBulk Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let bulkBody = BulkCreateHashTagsBody(
  tags = @["breaking", "world-news", "economy"],
  createdBy = "editor@dailynews.com",
  replaceExisting = false
)
let (response, httpResponse) = client.addHashTagsBulk(tenantId = "newsroom-tenant-42", bulkCreateHashTagsBody = bulkBody)
if response.isSome:
  let created = response.get()
  echo created
else:
  echo "AddHashTagsBulk failed:", httpResponse
[inline-code-end]

---