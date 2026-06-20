## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Antwort

Gibt zurück: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Beispiel

[inline-code-attrs-start title = 'delete_question_config Beispiel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Einrichtung der Autorisierung
FastCommentsClient.configure do |config|
  # Konfiguriere API-Schlüssel-Autorisierung: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Entfernen Sie das Kommentarzeichen in der folgenden Zeile, um ein Präfix für den API-Schlüssel festzulegen, z. B. 'Bearer' (Standardwert ist nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 

begin
  
  result = api_instance.delete_question_config(tenant_id, id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->delete_question_config: #{e}"
end
[inline-code-end]

---