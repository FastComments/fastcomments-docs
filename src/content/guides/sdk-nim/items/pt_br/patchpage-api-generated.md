## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| id | string | Não |  |
| updateAPIPageData | UpdateAPIPageData | Não |  |

## Resposta

Retorna: [`Option[PatchPageAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_patch_page_api_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de patchPage'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateData = UpdateAPIPageData(
  title = "Breaking: Major Event Update",
  urlId = "news/major-event-update",
  visible = true,
  tags = @["breaking", "headline"],
  sortOrder = 5
)

let (response, httpResponse) = client.patchPage(
  tenantId = "my-tenant-123",
  id = "news/major-event-update",
  updateAPIPageData = updateData
)

if response.isSome:
  let page = response.get()
  discard page
[inline-code-end]