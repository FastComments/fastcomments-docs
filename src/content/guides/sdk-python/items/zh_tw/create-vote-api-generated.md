## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | query | Yes |  |
| direction | string | query | Yes |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |

## 回應

返回：[`VoteResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/vote_response.py)

## 範例

[inline-code-attrs-start title = 'create_vote 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import CreateVoteOptions
from client.models.vote_response import VoteResponse
from client.rest import ApiException
from pprint import pprint

# 定義主機是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
# 用戶端必須依照 API 伺服器的安全政策設定驗證與授權參數。
# 下面提供每種認證方式的範例，請使用符合您認證需求的範例。

# 設定 API 金鑰授權：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 取消註解以下行以設定前置詞 (例如 Bearer) 給 API 金鑰，如有需要
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 以 API 客戶端實例進入上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    direction = 'direction_example' # str | 
    user_id = 'user_id_example' # str |  (可選)
    anon_user_id = 'anon_user_id_example' # str |  (可選)

    try:
        api_response = api_instance.create_vote(tenant_id, comment_id, direction, CreateVoteOptions(user_id=user_id, anon_user_id=anon_user_id))
        print("The response of DefaultApi->create_vote:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_vote: %s\n" % e)
[inline-code-end]