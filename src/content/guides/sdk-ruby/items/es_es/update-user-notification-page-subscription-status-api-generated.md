Habilitar o deshabilitar las notificaciones para una página. Cuando los usuarios están suscritos a una página, se crean notificaciones
para nuevos comentarios raíz, y también

## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | Yes |  |
| url | string | query | Yes |  |
| pageTitle | string | query | Yes |  |
| subscribedOrUnsubscribed | string | path | Yes |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/update_user_notification_status200_response.rb)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de update_user_notification_page_subscription_status'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Cadena | 
url_id = 'url_id_example' # Cadena | 
url = 'url_example' # Cadena | 
page_title = 'page_title_example' # Cadena | 
subscribed_or_unsubscribed = 'subscribe' # Cadena | 
opts = {
  sso: 'sso_example' # Cadena | 
}

begin
  
  result = api_instance.update_user_notification_page_subscription_status(tenant_id, url_id, url, page_title, subscribed_or_unsubscribed, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->update_user_notification_page_subscription_status: #{e}"
end
[inline-code-end]

---