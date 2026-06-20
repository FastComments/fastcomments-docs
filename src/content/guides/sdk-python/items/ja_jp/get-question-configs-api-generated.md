## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| skip | number | query | いいえ |  |

## レスポンス

返却: [`GetQuestionConfigsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_configs_response.py)

## 例

[inline-code-attrs-start title = 'get_question_configs の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_configs_response import GetQuestionConfigsResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は省略可能で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントはAPIサーバーのセキュリティポリシーに従って認証および認可のパラメータを設定する必要があります。
# 各認証方法の例を以下に示します。ご利用の認証ケースに合った例を使用してください。

# APIキー認証を設定する: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じてAPIキーのプレフィックス（例: Bearer）を設定するには下の行のコメントを外してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# APIクライアントのインスタンスを使用するコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # APIクラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  (オプション)

    try:
        api_response = api_instance.get_question_configs(tenant_id, skip=skip)
        print("The response of DefaultApi->get_question_configs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_configs: %s\n" % e)
[inline-code-end]