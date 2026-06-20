## Параметри

| Назва | Тип | Обов'язкове | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| createHashTagBody | CreateHashTagBody | Ні |  |

## Відповідь

Повертає: [`Option[CreateHashTagResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_hash_tag_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад addHashTag'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.addHashTag(tenantId = "my-tenant-123",
  createHashTagBody = CreateHashTagBody(name = "Breaking News",
    slug = "breaking-news",
    description = "Major breaking news items",
    color = "#ff0000",
    isTrending = true,
    aliases = @["breaking", "news"]))
if response.isSome:
  let created = response.get()
  echo created
[inline-code-end]

---