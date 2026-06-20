---
Comentadores anteriores en la página que NO están actualmente en línea. Ordenados por displayName.
Usar esto después de agotar /users/online para mostrar una sección "Members".
Paginación por cursor en commenterName: el servidor recorre el índice parcial {tenantId, urlId, commenterName} desde afterName hacia adelante mediante $gt, sin costo de $skip.

## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identificador de la URL de la página (limpiado en el servidor). |
| afterName | string | query | No | Cursor: proporcione nextAfterName de la respuesta anterior. |
| afterUserId | string | query | No | Desempate del cursor: proporcione nextAfterUserId de la respuesta anterior. Requerido cuando afterName está establecido para que los empates en el nombre no omitan entradas. |

## Respuesta

Devuelve: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_offline_response.rb)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_offline_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Identificador de la URL de la página (limpiado en el servidor).
opts = {
  after_name: 'after_name_example', # String | Cursor: proporcione nextAfterName de la respuesta anterior.
  after_user_id: 'after_user_id_example' # String | Desempate del cursor: proporcione nextAfterUserId de la respuesta anterior. Requerido cuando afterName está establecido para que los empates en el nombre no omitan entradas.
}

begin
  
  result = api_instance.get_offline_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_offline_users: #{e}"
end
[inline-code-end]

---