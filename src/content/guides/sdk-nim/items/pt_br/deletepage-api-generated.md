## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| id | string | Não |  |

## Resposta

Retorna: [`Option[DeletePageAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_page_api_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de deletePage'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deletePage(tenantId = "site-tenant-456", id = "news/winter-updates-2025")
if response.isSome:
  let deleted = response.get()
  echo "DeletePageAPIResponse:", deleted
else:
  echo "Delete failed, HTTP response:", httpResponse
[inline-code-end]

---