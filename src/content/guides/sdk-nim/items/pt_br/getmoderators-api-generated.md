## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| skip | float64 | Não |  |

## Resposta

Retorna: [`Option[GetModeratorsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_moderators_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getModerators'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getModerators(tenantId = "my-tenant-123", skip = 0.0)
if response.isSome:
  let moderators = response.get()
  echo "Received moderators response:", moderators
else:
  echo "No moderators returned"
[inline-code-end]

---