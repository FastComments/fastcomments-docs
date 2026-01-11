req
tenantId
urlId
userIdWS

## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| urlId | string | query | Sí |  |
| userIdWS | string | query | Sí |  |
| startTime | integer | query | Sí |  |
| endTime | integer | query | Sí |  |

## Respuesta

Devuelve: [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_event_log200_response.rb)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_event_log'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Cadena | 
url_id = 'url_id_example' # Cadena | 
user_id_ws = 'user_id_ws_example' # Cadena | 
start_time = 789 # Entero | 
end_time = 789 # Entero | 

begin
  
  result = api_instance.get_event_log(tenant_id, url_id, user_id_ws, start_time, end_time)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_event_log: #{e}"
end
[inline-code-end]

---