## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Ναι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_comment_ban_status_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_comment_ban_status'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # String | 
opts = {
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_comment_ban_status(comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_comment_ban_status: #{e}"
end
[inline-code-end]

---