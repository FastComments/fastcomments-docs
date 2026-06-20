## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

返却: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_user_search_response.rb)

## 例

[inline-code-attrs-start title = 'get_search_users の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  value: 'value_example', # 文字列 | 
  sso: 'sso_example' # 文字列 | 
}

begin
  
  result = api_instance.get_search_users(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_search_users: #{e}"
end
[inline-code-end]

---