## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| createHashTagBody | CreateHashTagBody | No |  |

## Risposta

Restituisce: [`Option[CreateHashTagResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_hash_tag_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di addHashTag'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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