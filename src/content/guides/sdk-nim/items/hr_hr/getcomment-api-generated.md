## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Ne |  |

## Odgovor

Vraća: [`Option[GetComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment200response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer getComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getComment(tenantId = "my-tenant-123", id = "cmt-987654321")
if response.isSome:
  let comment = response.get()
  echo comment
[inline-code-end]

---