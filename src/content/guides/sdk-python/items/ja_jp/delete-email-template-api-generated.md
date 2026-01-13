## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| id | string | path | はい |  |

## レスポンス

戻り値: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## 例

[inline-code-attrs-start title = 'delete_email_template の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。

# クライアントは API サーバーのセキュリティポリシーに従って認証および
# 認可パラメータを構成する必要があります。
# 以下に各認証方式の例を示します、 
# ご自身の認証ユースケースに合う例を使用してください。

# API キー認証を設定: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて API キーのプレフィックス（例: Bearer）を設定するには、以下のコメントを外してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスを用いてコンテキストを開始します
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.delete_email_template(tenant_id, id)
        print("The response of DefaultApi->delete_email_template:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_email_template: %s\n" % e)
[inline-code-end]