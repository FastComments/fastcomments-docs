## 回應

回傳：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## 範例

[inline-code-attrs-start title = 'logout_public 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new

begin
  
  result = api_instance.logout_public
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->logout_public: #{e}"
end
[inline-code-end]

---