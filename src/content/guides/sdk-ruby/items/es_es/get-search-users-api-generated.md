## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| value | string | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_user_search_response.rb)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_search_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  value: 'value_example', # Cadena | 
  sso: 'sso_example' # Cadena | 
}

begin
  
  result = api_instance.get_search_users(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_search_users: #{e}"
end
[inline-code-end]

---