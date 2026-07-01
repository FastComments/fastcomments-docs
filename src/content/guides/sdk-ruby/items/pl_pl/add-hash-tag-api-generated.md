## Parameters

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|--------------|----------|------|
| tenantId | string | query | Yes |  |

## Odpowiedź

Zwraca: [`CreateHashTagResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_hash_tag_response.rb)

## Przykład

[inline-code-attrs-start title = 'add_hash_tag Przykład'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# konfiguracja autoryzacji
FastCommentsClient.configure do |config|
  # Konfiguruj autoryzację klucza API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Odkomentuj poniższą linię, aby ustawić prefiks dla klucza API, np. 'Bearer' (defaults to nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
create_hash_tag_body = FastCommentsClient::CreateHashTagBody.new({tag: 'tag_example'}) # CreateHashTagBody | 

begin
  
  result = api_instance.add_hash_tag(tenant_id, create_hash_tag_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->add_hash_tag: #{e}"
end
[inline-code-end]