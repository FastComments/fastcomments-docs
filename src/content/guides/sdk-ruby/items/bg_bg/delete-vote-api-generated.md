## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| editKey | string | query | Не |  |

## Отговор

Връща: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/delete_comment_vote200_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример за delete_vote'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# настройка на упълномощаването
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Премахнете коментара от следния ред, за да зададете префикс за API ключа, напр. 'Bearer' (по подразбиране: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
opts = {
  edit_key: 'edit_key_example' # String | 
}

begin
  
  result = api_instance.delete_vote(tenant_id, id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->delete_vote: #{e}"
end
[inline-code-end]

---