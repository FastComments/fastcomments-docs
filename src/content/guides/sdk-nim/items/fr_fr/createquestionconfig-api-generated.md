## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| createQuestionConfigBody | CreateQuestionConfigBody | Non |  |

## Réponse

Retourne : [`Option[CreateQuestionConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_question_config_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de createQuestionConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createQuestionConfig(
  tenantId = "my-tenant-123",
  createQuestionConfigBody = CreateQuestionConfigBody(
    label = "Article Question",
    required = true,
    minLength = 20,
    maxLength = 1000,
    allowedTags = @["comment","question","feedback"],
    notifyModerators = false
  )
)
if response.isSome:
  let cfg = response.get()
  echo "Created question config id: ", cfg.id
[inline-code-end]

---