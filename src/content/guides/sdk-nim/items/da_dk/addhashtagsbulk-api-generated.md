## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | Nej |  |

## Svar

Returnerer: [`Option[BulkCreateHashTagsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_bulk_create_hash_tags_response.nim)

## Eksempel

[inline-code-attrs-start title = 'addHashTagsBulk Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.addHashTagsBulk(
  tenantId = "my-tenant-123",
  bulkCreateHashTagsBody = BulkCreateHashTagsBody(
    hashTags = @["news", "technology"]
  )
)

if optResp.isSome:
  let resp = optResp.get()
[inline-code-end]