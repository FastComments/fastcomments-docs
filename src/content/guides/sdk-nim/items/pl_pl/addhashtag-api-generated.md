## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| createHashTagBody | CreateHashTagBody | Nie |  |

## Odpowiedź

Zwraca: [`Option[CreateHashTagResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_hash_tag_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład addHashTag'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (hashTagOpt, httpResp) = client.addHashTag(
  tenantId = "my-tenant-123",
  createHashTagBody = CreateHashTagBody(),
)

if hashTagOpt.isSome:
  let tag = hashTagOpt.get()
[inline-code-end]