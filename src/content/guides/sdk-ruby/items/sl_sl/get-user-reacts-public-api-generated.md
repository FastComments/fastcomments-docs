## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| postIds | array | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vrne: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/user_reacts_response.rb)

## Primer

[inline-code-attrs-start title = 'get_user_reacts_public Primer'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Niz | 
opts = {
  post_ids: ['inner_example'], # Polje<String> | 
  sso: 'sso_example' # Niz | 
}

begin
  
  result = api_instance.get_user_reacts_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_user_reacts_public: #{e}"
end
[inline-code-end]

---