Información masiva de usuarios para un tenant. Dados los userIds, devuelve información para mostrar de User / SSOUser.
Utilizado por el widget de comentarios para enriquecer a los usuarios que acaban de aparecer a través de un evento de presencia.
Sin contexto de página: la privacidad se hace cumplir de forma uniforme (los perfiles privados están enmascarados).

## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|--------|------|----------|-----------|-------------|
| tenantId | string | ruta | Sí |  |
| ids | string | consulta | Sí | userIds separados por comas. |

## Respuesta

Devuelve: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_info_response.rb)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_users_info'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
ids = 'ids_example' # String | userIds separados por comas.

begin
  
  result = api_instance.get_users_info(tenant_id, ids)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_users_info: #{e}"
end
[inline-code-end]

---