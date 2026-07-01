Aggregates documents by grouping them (if groupBy is provided) and applying multiple operations.  
Different operations (e.g. sum, countDistinct, avg, etc.) are supported.

## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| parentTenantId | string | query | No |  |
| includeStats | boolean | query | No |  |

## レスポンス

返却: [`AggregateResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/aggregate_response.py)

## 例

[inline-code-attrs-start title = '集計例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import AggregateOptions
from client.models.aggregate_response import AggregateResponse
from client.models.aggregation_request import AggregationRequest
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# configuration.py でサポートされているすべての設定パラメータの一覧を確認できます

# クライアントは、APIサーバーのセキュリティポリシーに従って認証および認可パラメータを設定する必要があります
# APIサーバーのセキュリティポリシーに従って
# 各認証方式の例が以下に提供されています。適切な例を使用してください
# あなたの認証ユースケースに合致するもの

# APIキー認証を設定: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて、APIキーのプレフィックス（例: Bearer）を設定する場合は、以下のコメントを解除してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# APIクライアントのインスタンスでコンテキストを開始します
with client.ApiClient(configuration) as api_client:
    # APIクラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    aggregation_request = client.AggregationRequest() # AggregationRequest | 
    parent_tenant_id = 'parent_tenant_id_example' # str |  (optional)
    include_stats = True # bool |  (optional)

    try:
        api_response = api_instance.aggregate(tenant_id, aggregation_request, AggregateOptions(parent_tenant_id=parent_tenant_id, include_stats=include_stats))
        print("The response of DefaultApi->aggregate:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->aggregate: %s\n" % e)
[inline-code-end]