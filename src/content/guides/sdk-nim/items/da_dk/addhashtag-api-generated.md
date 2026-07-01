## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createHashTagBody | CreateHashTagBody | No |  |

## Respons

Returnerer: [`Option[CreateHashTagResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_hash_tag_response.nim)

## Eksempel

[inline-code-attrs-start title = 'addHashTag Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (hashTagOpt, httpResp) = client.addHashTag(
  tenantId = "my-tenant-123",
  createHashTagBody = CreateHashTagBody(),
)

if hashTagOpt.isSome:
  let tag = hashTagOpt.get()
[inline-code-end]