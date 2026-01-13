## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| email | string | path | はい |  |

## レスポンス

戻り値: [`GetSSOUserByEmailAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_sso_user_by_email_api_response.py)

## 例

[inline-code-attrs-start title = 'get_sso_user_by_email の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_sso_user_by_email_api_response import GetSSOUserByEmailAPIResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、既定値は https://fastcomments.com です
# サポートされているすべての構成パラメーターの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証および認可のパラメーターを設定する必要があります
# これは API サーバーのセキュリティポリシーに従って行ってください。
# 以下に各認証方式の例を示します。
# ご自身の認証ユースケースに合う例を使用してください。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて、API キーに対するプレフィックス（例: Bearer）を設定するには下の行のコメントアウトを解除してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスを使ってコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    email = 'email_example' # str | 

    try:
        api_response = api_instance.get_sso_user_by_email(tenant_id, email)
        print("The response of DefaultApi->get_sso_user_by_email:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_sso_user_by_email: %s\n" % e)
[inline-code-end]

---