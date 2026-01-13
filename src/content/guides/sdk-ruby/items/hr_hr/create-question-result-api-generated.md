## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |

## Odgovor

Vraća: [`CreateQuestionResult200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_question_result200_response.rb)

## Primjer

[inline-code-attrs-start title = 'Primjer create_question_result'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Postavljanje autorizacije
FastCommentsClient.configure do |config|
  # Konfigurirajte autorizaciju API ključa: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Uklonite komentar sa sljedećeg retka da postavite prefiks za API ključ, npr. 'Bearer' (zadano je nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
create_question_result_body = FastCommentsClient::CreateQuestionResultBody.new({url_id: 'url_id_example', value: 3.56, question_id: 'question_id_example'}) # CreateQuestionResultBody | 

begin
  
  result = api_instance.create_question_result(tenant_id, create_question_result_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_question_result: #{e}"
end
[inline-code-end]