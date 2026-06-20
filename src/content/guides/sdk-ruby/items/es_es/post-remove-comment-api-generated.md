## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | ruta | Sí |  |
| sso | string | consulta | No |  |

## Respuesta

Devuelve: [`PostRemoveCommentResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/post_remove_comment_response.rb)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de post_remove_comment'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # Cadena | 
opts = {
  sso: 'sso_example' # Cadena | 
}

begin
  
  result = api_instance.post_remove_comment(comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_remove_comment: #{e}"
end
[inline-code-end]