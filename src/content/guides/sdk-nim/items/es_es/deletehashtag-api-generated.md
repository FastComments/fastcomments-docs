## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| tag | string | No |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | No |  |

## Respuesta

Retorna: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de deleteHashTag'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResp, httpResp) = client.deleteHashTag(tenantId = "my-tenant-123", tag = "sports", deleteHashTagRequestBody = DeleteHashTagRequestBody())
if apiResp.isSome:
  let emptyResp = apiResp.get()
[inline-code-end]

---