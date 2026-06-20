---
Actualmente espectadores en línea de una página: personas cuya sesión websocket está suscrita a la página en este momento.
Devuelve anonCount + totalCount (suscriptores de la sala, incluidos los espectadores anónimos que no enumeramos).

## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identificador de URL de la página (limpiado en el servidor). |
| afterName | string | query | No | Cursor: pase nextAfterName de la respuesta anterior. |
| afterUserId | string | query | No | Desempate de cursor: pase nextAfterUserId de la respuesta anterior. Requerido cuando afterName está establecido para que los empates en nombres no hagan que se pierdan entradas. |

## Respuesta

Devuelve: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_online_response.rb)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_online_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Page URL identifier (cleaned server-side).
opts = {
  after_name: 'after_name_example', # String | Cursor: pase nextAfterName de la respuesta anterior.
  after_user_id: 'after_user_id_example' # String | Desempate de cursor: pase nextAfterUserId de la respuesta anterior. Requerido cuando afterName está establecido para que los empates en nombres no hagan que se pierdan entradas.
}

begin
  
  result = api_instance.get_online_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_online_users: #{e}"
end
[inline-code-end]

---