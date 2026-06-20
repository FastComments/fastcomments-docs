## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tag | string | Нет |  |
| tenantId | string | Да |  |
| updateHashTagBody | UpdateHashTagBody | Нет |  |

## Ответ

Возвращает: [`Option[UpdateHashTagResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_hash_tag_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример patchHashTag'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.patchHashTag(tag = "breaking-news", tenantId = "my-tenant-123", updateHashTagBody = UpdateHashTagBody())
if response.isSome:
  let updatedHashTag = response.get()
  echo updatedHashTag
[inline-code-end]

---