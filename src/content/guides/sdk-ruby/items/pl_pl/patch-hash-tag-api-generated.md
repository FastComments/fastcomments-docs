## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tag | string | path | Tak |  |
| tenantId | string | query | Nie |  |

## Odpowiedź

Zwraca: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/update_hash_tag_response.rb)

## Przykład

[inline-code-attrs-start title = 'Przykład patch_hash_tag'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# konfiguracja autoryzacji
FastCommentsClient.configure do |config|
  # konfiguracja autoryzacji kluczem API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Odkomentuj poniższą linię, aby ustawić prefiks dla klucza API, np. 'Bearer' (domyślnie nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tag = 'tag_example' # String | 
opts = {
  tenant_id: 'tenant_id_example', # String | 
  update_hash_tag_body: FastCommentsClient::UpdateHashTagBody.new # UpdateHashTagBody | 
}

begin
  
  result = api_instance.patch_hash_tag(tag, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->patch_hash_tag: #{e}"
end
[inline-code-end]

---