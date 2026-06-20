## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| userId | string | query | Nein |  |
| trustFactor | string | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/set_user_trust_factor_response.rb)

## Beispiel

[inline-code-attrs-start title = 'set_trust_factor Beispiel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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