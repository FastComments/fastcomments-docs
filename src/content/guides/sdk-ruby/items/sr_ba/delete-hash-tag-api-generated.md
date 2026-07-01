## Parameters

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| tag | string | path | Da |  |

## Odgovor

Vraća: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Primer

[inline-code-attrs-start title = 'delete_hash_tag Primer'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# postavljanje autorizacije
FastCommentsClient.configure do |config|
  # Konfiguracija autorizacije API ključa: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Odkomentarišite sljedeću liniju da postavite prefiks za API ključ, npr. 'Bearer' (podrazumevano je nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
tag = 'tag_example' # String | 
delete_hash_tag_request_body = FastCommentsClient::DeleteHashTagRequestBody.new # DeleteHashTagRequestBody | 

begin
  
  result = api_instance.delete_hash_tag(tenant_id, tag, delete_hash_tag_request_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->delete_hash_tag: #{e}"
end
[inline-code-end]

---