---
## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| createHashTagBody | CreateHashTagBody | Ne |  |

## Odgovor

Vraća: [`Option[CreateHashTagResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_hash_tag_response.nim)

## Primjer

[inline-code-attrs-start title = 'addHashTag Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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