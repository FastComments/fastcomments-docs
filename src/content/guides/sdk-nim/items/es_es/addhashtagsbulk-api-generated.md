## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | No |  |

## Respuesta

Devuelve: [`Option[BulkCreateHashTagsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_bulk_create_hash_tags_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo addHashTagsBulk'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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