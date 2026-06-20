## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| commentId | string | path | Da |  |
| sso | string | query | Ne |  |

## Odgovor

Vrača: [`BlockSuccess`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/block_success.rb)

## Primer

[inline-code-attrs-start title = 'Primer uporabe block_from_comment_public'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Niz | 
comment_id = 'comment_id_example' # Niz | 
public_block_from_comment_params = FastCommentsClient::PublicBlockFromCommentParams.new({comment_ids: ['comment_ids_example']}) # PublicBlockFromCommentParams | 
opts = {
  sso: 'sso_example' # Niz | 
}

begin
  
  result = api_instance.block_from_comment_public(tenant_id, comment_id, public_block_from_comment_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->block_from_comment_public: #{e}"
end
[inline-code-end]