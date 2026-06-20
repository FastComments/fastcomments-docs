## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| includeByUserIdAndEmail | boolean | query | 否 |  |
| includeByIP | boolean | query | 否 |  |
| includeByEmailDomain | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/bulk_pre_ban_summary.py)

## 示例

[inline-code-attrs-start title = 'post_bulk_pre_ban_summary 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.bulk_pre_ban_params import BulkPreBanParams
from client.models.bulk_pre_ban_summary import BulkPreBanSummary
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认为 https://fastcomments.com
# 有关所有支持的配置参数的列表，请参见 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 在上下文中使用 API 客户端实例
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的实例
    api_instance = client.ModerationApi(api_client)
    bulk_pre_ban_params = client.BulkPreBanParams() # BulkPreBanParams | 
    include_by_user_id_and_email = True # bool |  (可选)
    include_by_ip = True # bool |  (可选)
    include_by_email_domain = True # bool |  (可选)
    sso = 'sso_example' # str |  (可选)

    try:
        api_response = api_instance.post_bulk_pre_ban_summary(bulk_pre_ban_params, include_by_user_id_and_email=include_by_user_id_and_email, include_by_ip=include_by_ip, include_by_email_domain=include_by_email_domain, sso=sso)
        print("The response of ModerationApi->post_bulk_pre_ban_summary:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_bulk_pre_ban_summary: %s\n" % e)
[inline-code-end]

---