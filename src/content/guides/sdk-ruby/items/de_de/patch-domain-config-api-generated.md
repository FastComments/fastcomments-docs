## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| domainToUpdate | string | path | Ja |  |

## Antwort

Gibt zurück: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/patch_domain_config_response.rb)

## Beispiel

[inline-code-attrs-start title = 'patch_domain_config Beispiel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Autorisierung einrichten
FastCommentsClient.configure do |config|
  # API-Schlüssel-Authentifizierung konfigurieren: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Entkommentieren Sie die folgende Zeile, um ein Präfix für den API-Schlüssel festzulegen, z. B. 'Bearer' (Standard: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
domain_to_update = 'domain_to_update_example' # String | 
patch_domain_config_params = FastCommentsClient::PatchDomainConfigParams.new # PatchDomainConfigParams | 

begin
  
  result = api_instance.patch_domain_config(tenant_id, domain_to_update, patch_domain_config_params)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->patch_domain_config: #{e}"
end
[inline-code-end]