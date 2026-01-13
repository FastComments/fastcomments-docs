## Parameters

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| meta | string | query | Nein |  |
| skip | number | query | Nein |  |

## Antwort

Gibt zurück: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_tenants200_response.rb)

## Beispiel

[inline-code-attrs-start title = 'get_tenants Beispiel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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
opts = {
  meta: 'meta_example', # String | 
  skip: 1.2 # Float | 
}

begin
  
  result = api_instance.get_tenants(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_tenants: #{e}"
end
[inline-code-end]

---