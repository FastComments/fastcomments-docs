## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | ruta | Sí |  |
| includeEmail | boolean | consulta | No |  |
| includeIP | boolean | consulta | No |  |
| sso | string | consulta | No |  |

## Respuesta

Devuelve: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_a_p_i_comment_response.rb)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_moderation_comment'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # Cadena | 
opts = {
  include_email: true, # Booleano | 
  include_ip: true, # Booleano | 
  sso: 'sso_example' # Cadena | 
}

begin
  
  result = api_instance.get_moderation_comment(comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_moderation_comment: #{e}"
end
[inline-code-end]

---