## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| skip | number | query | Nein |  |

## Antwort

Gibt zurück: [`GetEmailTemplateRenderErrors200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_email_template_render_errors200_response.rb)

## Beispiel

[inline-code-attrs-start title = 'get_email_template_render_errors Beispiel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Autorisierung einrichten
FastCommentsClient.configure do |config|
  # API-Schlüssel-Autorisierung konfigurieren: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Entfernen Sie das Kommentarzeichen der folgenden Zeile, um ein Präfix für den API-Schlüssel festzulegen, z. B. 'Bearer' (Standard: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
opts = {
  skip: 1.2 # Float | 
}

begin
  
  result = api_instance.get_email_template_render_errors(tenant_id, id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_email_template_render_errors: #{e}"
end
[inline-code-end]

---