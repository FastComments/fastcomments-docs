## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Non |  |

## Réponse

Renvoie : [`Option[GetQuestionConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_config_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de getQuestionConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getQuestionConfig(tenantId = "my-tenant-123", id = "qst-456")
if response.isSome:
  let cfg = response.get()
  discard cfg
[inline-code-end]

---