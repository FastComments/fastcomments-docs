## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|-------------|
| tenantId | string | Sí |  |
| options | GetSearchCommentsSummaryOptions | No |  |

## Respuesta

Devuelve: [`Option[ModerationCommentSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_comment_search_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getSearchCommentsSummary'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (summaryOpt, httpResp) = client.getSearchCommentsSummary(
  tenantId = "my-tenant-123",
  options = GetSearchCommentsSummaryOptions()
)

if summaryOpt.isSome:
  let summary = summaryOpt.get()
  echo summary
[inline-code-end]