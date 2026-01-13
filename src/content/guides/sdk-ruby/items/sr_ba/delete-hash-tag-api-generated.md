## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tag | string | path | Da |  |
| tenantId | string | query | Ne |  |

## Odgovor

Vraća: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/flag_comment_public200_response.rb)

## Primjer

[inline-code-attrs-start title = 'Primjer delete_hash_tag'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# postavljanje autorizacije
FastCommentsClient.configure do |config|
  # Konfigurišite autorizaciju API ključa: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Otkomentarišite sljedeću liniju da postavite prefiks za API ključ, npr. 'Bearer' (zadano: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tag = 'tag_example' # String | 
opts = {
  tenant_id: 'tenant_id_example', # String | 
  delete_hash_tag_request: FastCommentsClient::DeleteHashTagRequest.new # DeleteHashTagRequest | 
}

begin
  
  result = api_instance.delete_hash_tag(tag, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->delete_hash_tag: #{e}"
end
[inline-code-end]

---