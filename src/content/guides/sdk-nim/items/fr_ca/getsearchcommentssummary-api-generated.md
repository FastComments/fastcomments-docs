## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| options | GetSearchCommentsSummaryOptions | Non |  |

## Réponse

Retourne : [`Option[ModerationCommentSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_comment_search_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getSearchCommentsSummary'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (summaryOpt, httpResp) = client.getSearchCommentsSummary(
  tenantId = "my-tenant-123",
  options = GetSearchCommentsSummaryOptions()
)

if summaryOpt.isSome:
  let summary = summaryOpt.get()
  echo summary
[inline-code-end]

---