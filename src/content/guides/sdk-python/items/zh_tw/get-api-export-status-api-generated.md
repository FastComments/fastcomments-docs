## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| batchJobId | string | 查詢 | 否 |  |
| sso | string | 查詢 | 否 |  |

## 回應

回傳: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_export_status_response.py)

## 範例

[inline-code-attrs-start title = 'get_api_export_status 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_export_status_response import ModerationExportStatusResponse
from client.rest import ApiException
from pprint import pprint

# 定義 host 為選填，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 進入包含 API client 實例的上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.ModerationApi(api_client)
    batch_job_id = 'batch_job_id_example' # str |  (選填)
    sso = 'sso_example' # str |  (選填)

    try:
        api_response = api_instance.get_api_export_status(batch_job_id=batch_job_id, sso=sso)
        print("The response of ModerationApi->get_api_export_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_api_export_status: %s\n" % e)
[inline-code-end]