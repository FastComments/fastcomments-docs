## パラメータ

| 名前 | Type | 位置 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| skip | number | query | いいえ |  |

## レスポンス

戻り値: [`GetQuestionConfigs200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_configs200_response.py)

## 例

[inline-code-attrs-start title = 'get_question_configs の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_configs200_response import GetQuestionConfigs200Response
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは API サーバーのセキュリティポリシーに従って認証および認可のパラメータを設定する必要があります。
# 各認証方式の例は下記に示します。自身のユースケースに合った例を使用してください。

# API キー認証を設定します: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要であれば API キーのプレフィックス（例: Bearer）を設定するため、以下のコメントを外してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスとともにコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
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

---