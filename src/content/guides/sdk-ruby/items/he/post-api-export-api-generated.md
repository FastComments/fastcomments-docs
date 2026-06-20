## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| text-search | string | query | לא |  |
| byIPFromComment | string | query | לא |  |
| filters | string | query | לא |  |
| searchFilters | string | query | לא |  |
| sorts | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_export_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-post_api_export'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  text_search: 'text_search_example', # מחרוזת | 
  by_ip_from_comment: 'by_ip_from_comment_example', # מחרוזת | 
  filters: 'filters_example', # מחרוזת | 
  search_filters: 'search_filters_example', # מחרוזת | 
  sorts: 'sorts_example', # מחרוזת | 
  sso: 'sso_example' # מחרוזת | 
}

begin
  
  result = api_instance.post_api_export(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_api_export: #{e}"
end
[inline-code-end]

---