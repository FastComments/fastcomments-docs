## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| includeByUserIdAndEmail | boolean | query | いいえ |  |
| includeByIP | boolean | query | いいえ |  |
| includeByEmailDomain | boolean | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/bulk_pre_ban_summary.rb)

## 例

[inline-code-attrs-start title = 'post_bulk_pre_ban_summary の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
bulk_pre_ban_params = FastCommentsClient::BulkPreBanParams.new({comment_ids: ['comment_ids_example']}) # BulkPreBanParams | 
opts = {
  include_by_user_id_and_email: true, # Boolean | 
  include_by_ip: true, # Boolean | 
  include_by_email_domain: true, # Boolean | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.post_bulk_pre_ban_summary(bulk_pre_ban_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_bulk_pre_ban_summary: #{e}"
end
[inline-code-end]