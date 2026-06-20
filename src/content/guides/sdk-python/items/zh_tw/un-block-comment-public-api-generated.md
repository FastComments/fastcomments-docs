## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| commentId | string | path | 是 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/unblock_success.py)

## 範例

[inline-code-attrs-start title = 'un_block_comment_public 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.public_block_from_comment_params import PublicBlockFromCommentParams
from client.models.unblock_success import UnblockSuccess
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# 定義 host 為選用，預設為 https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
# 請參閱 configuration.py 以取得所有支援的設定參數清單。

configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
# 使用 API client 的實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # 建立 API 類別的實例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    public_block_from_comment_params = client.PublicBlockFromCommentParams() # PublicBlockFromCommentParams | 
    sso = 'sso_example' # str |  （選用）

    try:
        api_response = api_instance.un_block_comment_public(tenant_id, comment_id, public_block_from_comment_params, sso=sso)
        print("The response of PublicApi->un_block_comment_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->un_block_comment_public: %s\n" % e)
[inline-code-end]