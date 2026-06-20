## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |

## Odgovor

Vrne: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Primer

[inline-code-attrs-start title = 'delete_question_result Primer'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# nastavitev avtorizacije
FastCommentsClient.configure do |config|
  # Konfiguriraj avtorizacijo API ključa: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Odkomentirajte naslednjo vrstico, da nastavite predpono za API ključ, npr. 'Bearer' (privzeto nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 

begin
  
  result = api_instance.delete_question_result(tenant_id, id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->delete_question_result: #{e}"
end
[inline-code-end]

---