---
## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| deleteComments | boolean | query | No |  |
| commentDeleteMode | string | query | No |  |

## レスポンス

戻り値: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_sso_user_api_response.py)

## 例

[inline-code-attrs-start title = 'delete_sso_user の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_sso_user_api_response import DeleteSSOUserAPIResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての構成パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは API サーバーのセキュリティポリシーに従って認証および認可パラメータを設定する必要があります。
# 各認証方式の例を以下に示します、
# ご自身の認証ユースケースに合う例を使用してください。
# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスでコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    delete_comments = True # bool |  （任意）
    comment_delete_mode = 'comment_delete_mode_example' # str |  （任意）

    try:
        api_response = api_instance.delete_sso_user(tenant_id, id, delete_comments=delete_comments, comment_delete_mode=comment_delete_mode)
        print("The response of DefaultApi->delete_sso_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_sso_user: %s\n" % e)
[inline-code-end]

---