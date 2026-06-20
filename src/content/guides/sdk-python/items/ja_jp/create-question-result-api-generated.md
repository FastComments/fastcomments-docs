## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |

## レスポンス

戻り値: [`CreateQuestionResultResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_question_result_response.py)

## 例

[inline-code-attrs-start title = 'create_question_result の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_question_result_body import CreateQuestionResultBody
from client.models.create_question_result_response import CreateQuestionResultResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証および認可パラメータを設定する必要があります
# これは API サーバーのセキュリティポリシーに従って行ってください。
# 以下に各認証方法の例を示します。 
# ご自身の認証ユースケースに合う例を使用してください。

# API キー認証を設定: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて API キーのプレフィックス（例: Bearer）を設定するには以下の行のコメントを外してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスでコンテキストを開始します
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを生成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_question_result_body = client.CreateQuestionResultBody() # CreateQuestionResultBody | 

    try:
        api_response = api_instance.create_question_result(tenant_id, create_question_result_body)
        print("The response of DefaultApi->create_question_result:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_question_result: %s\n" % e)
[inline-code-end]