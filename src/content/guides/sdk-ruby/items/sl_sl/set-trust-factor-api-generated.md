## Parametri

| Name | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| userId | string | query | Ne |  |
| trustFactor | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odziv

Vrača: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/set_user_trust_factor_response.rb)

## Primer

[inline-code-attrs-start title = 'Primer set_trust_factor'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  user_id: 'user_id_example', # Niz | 
  trust_factor: 'trust_factor_example', # Niz | 
  sso: 'sso_example' # Niz | 
}

begin
  
  result = api_instance.set_trust_factor(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->set_trust_factor: #{e}"
end
[inline-code-end]