## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| locale | string | query | 否 |  |

## 回應

回傳： [`RenderEmailTemplate200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/render_email_template200_response.py)

## 範例

[inline-code-attrs-start title = 'render_email_template 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.render_email_template200_response import RenderEmailTemplate200Response
from client.models.render_email_template_body import RenderEmailTemplateBody
from client.rest import ApiException
from pprint import pprint

# 定義主機是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須根據 API 伺服器的安全政策設定驗證與授權參數。
# 下方提供了每種驗證方法的範例，請使用符合您驗證需求的範例。

# 設定 API key 授權：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消註解以下內容以設定 API key 的前綴（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 建立一個 API client 的上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    render_email_template_body = client.RenderEmailTemplateBody() # RenderEmailTemplateBody | 
    locale = 'locale_example' # str |  (optional)

    try:
        api_response = api_instance.render_email_template(tenant_id, render_email_template_body, locale=locale)
        print("The response of DefaultApi->render_email_template:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->render_email_template: %s\n" % e)
[inline-code-end]

---