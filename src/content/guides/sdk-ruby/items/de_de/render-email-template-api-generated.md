## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| locale | string | query | Nein |  |

## Antwort

Gibt zurück: [`RenderEmailTemplateResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/render_email_template_response.rb)

## Beispiel

[inline-code-attrs-start title = 'render_email_template Beispiel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Autorisierung einrichten
FastCommentsClient.configure do |config|
  # API-Schlüssel-Authentifizierung konfigurieren: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Entkommentieren Sie die folgende Zeile, um ein Präfix für den API-Schlüssel festzulegen, z. B. 'Bearer' (Standardmäßig: nil)
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