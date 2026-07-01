## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| id | string | Não |  |
| updateAPIPageData | UpdateAPIPageData | Não |  |

## Resposta

Retorna: [`Option[PatchPageAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_patch_page_api_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo patchPage'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.patchPage(
  tenantId = "my-tenant-123",
  id = "news/article-456",
  updateAPIPageData = UpdateAPIPageData(title = "Updated article title", description = "Revised description")
)

if response.isSome:
  let resp = response.get()
[inline-code-end]