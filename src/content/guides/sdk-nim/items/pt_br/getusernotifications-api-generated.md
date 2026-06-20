## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| pageSize | int | Não |  |
| afterId | string | Não |  |
| includeContext | bool | Não |  |
| afterCreatedAt | int64 | Não |  |
| unreadOnly | bool | Não |  |
| dmOnly | bool | Não |  |
| noDm | bool | Não |  |
| includeTranslations | bool | Não |  |
| includeTenantNotifications | bool | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`Option[GetMyNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_my_notifications_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getUserNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserNotifications(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  pageSize = 0,
  afterId = "",
  includeContext = false,
  afterCreatedAt = 0,
  unreadOnly = false,
  dmOnly = false,
  noDm = false,
  includeTranslations = false,
  includeTenantNotifications = false,
  sso = ""
)

if response.isSome:
  let notifications = response.get()
  echo notifications
[inline-code-end]

---