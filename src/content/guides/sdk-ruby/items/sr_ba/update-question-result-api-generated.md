## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |

## Odgovor

Vraća: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Primjer

[inline-code-attrs-start title = 'update_question_result Primjer'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# postavljanje autorizacije
FastCommentsClient.configure do |config|
  # Konfigurišite autorizaciju API ključa: api_key
  # Odkomentarišite sljedeću liniju da postavite prefiks za API ključ, npr. 'Bearer' (podrazumijevano je nil)
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
update_question_result_body = FastCommentsClient::UpdateQuestionResultBody.new # UpdateQuestionResultBody | 

begin
  
  result = api_instance.update_question_result(tenant_id, id, update_question_result_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->update_question_result: #{e}"
end
[inline-code-end]