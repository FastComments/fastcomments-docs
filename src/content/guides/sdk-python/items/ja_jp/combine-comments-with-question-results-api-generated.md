## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| questionId | string | query | いいえ |  |
| questionIds | array | query | いいえ |  |
| urlId | string | query | いいえ |  |
| startDate | string | query | いいえ |  |
| forceRecalculate | boolean | query | いいえ |  |
| minValue | number | query | いいえ |  |
| maxValue | number | query | いいえ |  |
| limit | number | query | いいえ |  |

## レスポンス

戻り値: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/combine_comments_with_question_results200_response.py)

## 例

[inline-code-attrs-start title = 'combine_comments_with_question_results の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.combine_comments_with_question_results200_response import CombineCommentsWithQuestionResults200Response
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証および認可パラメータを設定する必要があります
# API サーバーのセキュリティポリシーに従ってください。
# 以下に各認証方式の例を示します。
# 自分の認証ユースケースに合う例を使用してください。

# APIキー認証を設定: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて下の行のコメントアウトを解除して API キーのプレフィックス（例: Bearer）を設定してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスを使ってコンテキストを開始します
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    question_id = 'question_id_example' # str |  (任意)
    question_ids = ['question_ids_example'] # List[str] |  (任意)
    url_id = 'url_id_example' # str |  (任意)
    start_date = '2013-10-20T19:20:30+01:00' # datetime |  (任意)
    force_recalculate = True # bool |  (任意)
    min_value = 3.4 # float |  (任意)
    max_value = 3.4 # float |  (任意)
    limit = 3.4 # float |  (任意)

    try:
        api_response = api_instance.combine_comments_with_question_results(tenant_id, question_id=question_id, question_ids=question_ids, url_id=url_id, start_date=start_date, force_recalculate=force_recalculate, min_value=min_value, max_value=max_value, limit=limit)
        print("The response of DefaultApi->combine_comments_with_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->combine_comments_with_question_results: %s\n" % e)
[inline-code-end]