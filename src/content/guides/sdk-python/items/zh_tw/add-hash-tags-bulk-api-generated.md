## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 否 |  |

## 回應

回傳：[`BulkCreateHashTagsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/bulk_create_hash_tags_response.py)

## 範例

[inline-code-attrs-start title = 'add_hash_tags_bulk 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.bulk_create_hash_tags_body import BulkCreateHashTagsBody
from client.models.bulk_create_hash_tags_response import BulkCreateHashTagsResponse
from client.rest import ApiException
from pprint import pprint

# 定義 host 為選填，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須設定驗證和授權參數
# 以符合 API 伺服器的安全政策。
# 下方提供每種驗證方法的範例，請使用
# 符合您驗證使用情境的範例。
# 設定 API 金鑰授權：api_key
# 若需要，取消註解下方以設定 API 金鑰的前綴（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 客戶端實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str |  (選填)
    bulk_create_hash_tags_body = client.BulkCreateHashTagsBody() # BulkCreateHashTagsBody |  (選填)

    try:
        api_response = api_instance.add_hash_tags_bulk(tenant_id=tenant_id, bulk_create_hash_tags_body=bulk_create_hash_tags_body)
        print("The response of DefaultApi->add_hash_tags_bulk:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_hash_tags_bulk: %s\n" % e)
[inline-code-end]