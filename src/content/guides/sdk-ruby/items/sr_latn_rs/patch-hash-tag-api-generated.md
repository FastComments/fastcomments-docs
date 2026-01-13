## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tag | string | path | Da |  |
| tenantId | string | query | Ne |  |

## Odgovor

Vraća: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/patch_hash_tag200_response.rb)

## Primer

[inline-code-attrs-start title = 'patch_hash_tag Primer'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Podešavanje autorizacije
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Otkomentarišite sledeću liniju da biste postavili prefiks za API ključ, npr. 'Bearer' (podrazumevano nil)
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