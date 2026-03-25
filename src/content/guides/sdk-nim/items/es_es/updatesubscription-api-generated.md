## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | No |  |
| updateAPIUserSubscriptionData | UpdateAPIUserSubscriptionData | No |  |
| userId | string | No |  |

## Respuesta

Devuelve: [`Option[UpdateSubscriptionAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_subscription_api_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de updateSubscription'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateSubscription(
  tenantId = "my-tenant-123",
  id = "sub-456",
  updateAPIUserSubscriptionData = UpdateAPIUserSubscriptionData(
    subscribed = true,
    channels = @["email", "push"]
  ),
  userId = "user-789"
)

if response.isSome:
  let updated = response.get()
  echo "Subscription updated:", updated
else:
  echo "Update failed, HTTP response:", httpResponse
[inline-code-end]

---