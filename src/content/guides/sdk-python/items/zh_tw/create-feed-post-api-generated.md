## 參數

| 名稱 | 型別 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| broadcastId | string | query | 否 |  |
| isLive | boolean | query | 否 |  |
| doSpamCheck | boolean | query | 否 |  |
| skipDupCheck | boolean | query | 否 |  |

## 回應

返回：[`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_feed_posts_response.py)

## 示例

[inline-code-attrs-start title = 'create_feed_post 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import CreateFeedPostOptions
from client.models.create_feed_post_params import CreateFeedPostParams
from client.models.create_feed_posts_response import CreateFeedPostsResponse
from client.rest import ApiException
from pprint import pprint

# 定義主機是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數列表。
# 客戶端必須依照 API 伺服器的安全政策設定驗證與授權參數。
# 下面提供每種驗證方式的範例，請使用符合您驗證需求的範例。

# 設定 API 金鑰授權：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如需為 API 金鑰設定前綴（例如 Bearer），請取消註解下列程式碼
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 以 API 客戶端實例建立執行環境
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_feed_post_params = client.CreateFeedPostParams() # CreateFeedPostParams | 
    broadcast_id = 'broadcast_id_example' # str |  (optional)
    is_live = True # bool |  (optional)
    do_spam_check = True # bool |  (optional)
    skip_dup_check = True # bool |  (optional)

    try:
        api_response = api_instance.create_feed_post(tenant_id, create_feed_post_params, CreateFeedPostOptions(broadcast_id=broadcast_id, is_live=is_live, do_spam_check=do_spam_check, skip_dup_check=skip_dup_check))
        print("DefaultApi->create_feed_post 的回應結果：\n")
        pprint(api_response)
    except Exception as e:
        print("呼叫 DefaultApi->create_feed_post 時發生例外情況：%s\n" % e)
[inline-code-end]