---
## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| locale | string | query | No |  |

## Respuesta

Devuelve: [`RenderEmailTemplateResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/render_email_template_response.rb)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de render_email_template'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# configurar la autorización
FastCommentsClient.configure do |config|
  # Configurar la autorización de la clave API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Descomenta la siguiente línea para establecer un prefijo para la clave API, por ejemplo 'Bearer' (por defecto nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
render_email_template_body = FastCommentsClient::RenderEmailTemplateBody.new({email_template_id: 'email_template_id_example', ejs: 'ejs_example'}) # RenderEmailTemplateBody | 
opts = {
  locale: 'locale_example' # String | 
}

begin
  
  result = api_instance.render_email_template(tenant_id, render_email_template_body, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->render_email_template: #{e}"
end
[inline-code-end]

---