## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |

## Antwort

Gibt zurück: [`CreateEmailTemplateResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_email_template_response.rb)

## Beispiel

[inline-code-attrs-start title = 'create_email_template Beispiel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Autorisierung einrichten
FastCommentsClient.configure do |config|
  # API-Schlüssel-Authentifizierung konfigurieren: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Heben Sie die folgende Zeile aus, um ein Präfix für den API-Schlüssel festzulegen, z. B. 'Bearer' (Standard: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
create_email_template_body = FastCommentsClient::CreateEmailTemplateBody.new({email_template_id: 'email_template_id_example', display_name: 'display_name_example', ejs: 'ejs_example'}) # CreateEmailTemplateBody | 

begin
  
  result = api_instance.create_email_template(tenant_id, create_email_template_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_email_template: #{e}"
end
[inline-code-end]