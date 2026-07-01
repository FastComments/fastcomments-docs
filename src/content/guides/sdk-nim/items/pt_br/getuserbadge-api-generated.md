## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## Resposta

Retorna: [`Option[APIGetUserBadgeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_user_badge_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getUserBadge'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (badgeOpt, httpResp) = client.getUserBadge(tenantId = "my-tenant-123", id = "user-789")
if badgeOpt.isSome:
  let badge = badgeOpt.get()
  echo badge
else:
  echo "No badge found"
echo httpResp.statusCode
[inline-code-end]

---