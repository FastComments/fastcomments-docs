## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | poizvedba | Da |  |
| tag | string | pot | Da |  |

## Odgovor

Vrne: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Primer

[inline-code-attrs-start title = 'delete_hash_tag Primer'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# nastavi pooblastilo
FastCommentsClient.configure do |config|
  # Nastavi pooblastilo ključ API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Odkomentirajte naslednjo vrstico, da nastavite predpono za ključ API, npr. 'Bearer' (privzeto nil)
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