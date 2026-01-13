## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |

## Respons

Returnerer: [`CreateQuestionConfig200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_question_config200_response.rb)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på create_question_config'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# opsætning af autorisation
FastCommentsClient.configure do |config|
  # Konfigurer API-nøgleautorisation: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Fjern kommentaren fra følgende linje for at angive et præfiks for API-nøglen, f.eks. 'Bearer' (standard er nil)
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