ドキュメントをグループ化（groupBy が指定されている場合）して複数の操作を適用し、集計を行います。sum、countDistinct、avg など、さまざまな操作がサポートされています。

## パラメータ

| 名称 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| parentTenantId | string | query | No |  |
| includeStats | boolean | query | No |  |

## レスポンス

戻り値: [`AggregateResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/aggregate_response.py)

## 例

[inline-code-attrs-start title = 'aggregate の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.aggregate_response import AggregateResponse
from client.models.aggregation_request import AggregationRequest
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証および認可のパラメータを設定する必要があります
# これは API サーバーのセキュリティポリシーに従って行ってください。
# 以下に各認証方式の例を示します。ご使用の認証ケースに合った例を使用してください。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要であれば、API キー用のプレフィックス（例: Bearer）を設定するために下の行のコメントを外してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスを使ってコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    aggregation_request = client.AggregationRequest() # AggregationRequest | 
    parent_tenant_id = 'parent_tenant_id_example' # str |  （任意）
    include_stats = True # bool |  （任意）

    try:
        api_response = api_instance.aggregate(tenant_id, aggregation_request, parent_tenant_id=parent_tenant_id, include_stats=include_stats)
        print("The response of DefaultApi->aggregate:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->aggregate: %s\n" % e)
[inline-code-end]