## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tag | string | Não |  |
| tenantId | string | Sim |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | Não |  |

## Resposta

Retorna: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de deleteHashTag'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteHashTag(
  tag = "",
  tenantId = "my-tenant-123",
  deleteHashTagRequestBody = DeleteHashTagRequestBody()
)

if response.isSome:
  let emptyResp = response.get()
  echo "Deleted hashtag for tenant my-tenant-123; response:", $emptyResp, " status:", $httpResponse.status
else:
  echo "No response body; status:", $httpResponse.status
[inline-code-end]

---