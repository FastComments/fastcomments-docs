## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| search | string | query | כן |  |
| locale | string | query | לא |  |
| rating | string | query | לא |  |
| page | number | query | לא |  |

## תגובה

מחזיר: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_gifs_search_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_gifs_search'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
search = 'search_example' # String | 
opts = {
  locale: 'locale_example', # String | 
  rating: 'rating_example', # String | 
  page: 1.2 # Float | 
}

begin
  
  result = api_instance.get_gifs_search(tenant_id, search, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_gifs_search: #{e}"
end
[inline-code-end]

---