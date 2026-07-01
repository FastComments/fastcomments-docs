## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|------|------|----------|------------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_comment_text_response.rb)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo get_moderation_comment_text'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # Cadena | 
comment_id = 'comment_id_example' # Cadena | 
opts = {
  sso: 'sso_example' # Cadena | 
}

begin
  
  result = api_instance.get_moderation_comment_text(tenant_id, comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error al llamar ModerationApi->get_moderation_comment_text: #{e}"
end
[inline-code-end]