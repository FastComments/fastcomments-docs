## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |

## Antwort

Gibt zurück: [`CreateQuestionConfig200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_question_config200_response.rb)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für create_question_config'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Autorisierung einrichten
FastCommentsClient.configure do |config|
  # API-Schlüssel-Autorisierung konfigurieren: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Entkommentieren Sie die folgende Zeile, um ein Präfix für den API-Schlüssel festzulegen, z. B. 'Bearer' (Standard: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
create_question_config_body = FastCommentsClient::CreateQuestionConfigBody.new({name: 'name_example', question: 'question_example', type: 'type_example', reporting_order: 3.56}) # CreateQuestionConfigBody | 

begin
  
  result = api_instance.create_question_config(tenant_id, create_question_config_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_question_config: #{e}"
end
[inline-code-end]

---