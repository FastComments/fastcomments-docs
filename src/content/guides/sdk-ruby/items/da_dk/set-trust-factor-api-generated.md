## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| userId | string | query | Nej |  |
| trustFactor | string | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/set_user_trust_factor_response.rb)

## Eksempel

[inline-code-attrs-start title = 'set_trust_factor Eksempel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  user_id: 'user_id_example', # String | 
  trust_factor: 'trust_factor_example', # String | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.set_trust_factor(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->set_trust_factor: #{e}"
end
[inline-code-end]

---