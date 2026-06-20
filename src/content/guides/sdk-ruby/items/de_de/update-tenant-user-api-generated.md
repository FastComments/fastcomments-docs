## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| updateComments | string | query | Nein |  |

## Antwort

Gibt zurück: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Beispiel

[inline-code-attrs-start title = 'update_tenant_user Beispiel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Autorisierung einrichten
FastCommentsClient.configure do |config|
  # API-Schlüssel-Autorisierung konfigurieren: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Kommentierung der folgenden Zeile aufheben, um ein Präfix für den API-Schlüssel zu setzen, z. B. 'Bearer' (Standard ist nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
update_tenant_user_body = FastCommentsClient::UpdateTenantUserBody.new # UpdateTenantUserBody | 
opts = {
  update_comments: 'update_comments_example' # String | 
}

begin
  
  result = api_instance.update_tenant_user(tenant_id, id, update_tenant_user_body, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->update_tenant_user: #{e}"
end
[inline-code-end]