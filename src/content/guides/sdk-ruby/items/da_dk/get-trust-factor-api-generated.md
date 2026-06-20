## Parametre

| Navn | Type | Placering | Obligatorisk | Beskrivelse |
|------|------|----------|----------|-------------|
| userId | string | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_user_trust_factor_response.rb)

## Eksempel

[inline-code-attrs-start title = 'get_trust_factor Example'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  user_id: 'user_id_example', # Streng | 
  sso: 'sso_example' # Streng | 
}

begin
  
  result = api_instance.get_trust_factor(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_trust_factor: #{e}"
end
[inline-code-end]

---