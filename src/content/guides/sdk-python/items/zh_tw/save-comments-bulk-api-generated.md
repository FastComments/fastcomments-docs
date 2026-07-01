## Parameters

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|------|-------------|
| tenantId | string | query | 是 |  |
| isLive | boolean | query | 否 |  |
| doSpamCheck | boolean | query | 否 |  |
| sendEmails | boolean | query | 否 |  |
| populateNotifications | boolean | query | 否 |  |

## Response

返回：[`SaveCommentsBulkResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/save_comments_bulk_response.py)

## Example

[inline-code-attrs-start title = 'save_comments_bulk 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import SaveCommentsBulkOptions
from client.models.create_comment_params import CreateCommentParams
from client.models.save_comments_bulk_response import SaveCommentsBulkResponse
from client.rest import ApiException
from pprint import pprint

# 定義主機是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客戶端必須依照 API 伺服器的安全策略設定驗證與授權參數
# 以下提供每種驗證方式的範例，請使用符合您驗證需求的範例。

# 設定 API Key 授權: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消註解以下行以設定 API Key 的前綴（例如 Bearer） for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 客戶端實例進入上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_comment_params = [client.CreateCommentParams()] # List[CreateCommentParams] | 
    is_live = True # bool |  (optional)
    do_spam_check = True # bool |  (optional)
    send_emails = True # bool |  (optional)
    populate_notifications = True # bool |  (optional)

    try:
        api_response = api_instance.save_comments_bulk(tenant_id, create_comment_params, SaveCommentsBulkOptions(is_live=is_live, do_spam_check=do_spam_check, send_emails=send_emails, populate_notifications=populate_notifications))
        print("The response of DefaultApi->save_comments_bulk:\\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->save_comments_bulk: %s\\n" % e)
[inline-code-end]