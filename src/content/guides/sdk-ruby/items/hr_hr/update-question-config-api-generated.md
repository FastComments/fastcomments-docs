## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |

## Odgovor

Vraća: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/flag_comment_public200_response.rb)

## Primjer

[inline-code-attrs-start title = 'Primjer update_question_config'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# postavljanje autorizacije
FastCommentsClient.configure do |config|
  # Konfigurirajte autorizaciju API ključa: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Otkomentirajte sljedeći redak da biste postavili prefiks za API ključ, npr. 'Bearer' (zadano je nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
update_question_config_body = FastCommentsClient::UpdateQuestionConfigBody.new # UpdateQuestionConfigBody | 

begin
  
  result = api_instance.update_question_config(tenant_id, id, update_question_config_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->update_question_config: #{e}"
end
[inline-code-end]