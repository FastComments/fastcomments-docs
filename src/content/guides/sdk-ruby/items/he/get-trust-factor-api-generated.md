## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| userId | string | query | לא |  |
| sso | string | query | לא |  |

## תשובה

מחזיר: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_user_trust_factor_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמת get_trust_factor'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  user_id: 'user_id_example', # מחרוזת | 
  sso: 'sso_example' # מחרוזת | 
}

begin
  
  result = api_instance.get_trust_factor(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_trust_factor: #{e}"
end
[inline-code-end]

---