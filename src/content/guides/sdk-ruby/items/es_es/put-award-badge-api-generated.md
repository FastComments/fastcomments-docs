## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| badgeId | string | query | Sí |  |
| userId | string | query | No |  |
| commentId | string | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/award_user_badge_response.rb)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo put_award_badge'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # Cadena | 
badge_id = 'badge_id_example' # Cadena | 
opts = {
  user_id: 'user_id_example', # Cadena | 
  comment_id: 'comment_id_example', # Cadena | 
  broadcast_id: 'broadcast_id_example', # Cadena | 
  sso: 'sso_example' # Cadena | 
}

begin
  
  result = api_instance.put_award_badge(tenant_id, badge_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->put_award_badge: #{e}"
end
[inline-code-end]