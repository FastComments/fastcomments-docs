## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| isLive | boolean | query | No |  |
| doSpamCheck | boolean | query | No |  |
| sendEmails | boolean | query | No |  |
| populateNotifications | boolean | query | No |  |

## レスポンス

戻り値: [`SaveCommentsBulkResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/save_comments_bulk_response.py)

## 例

[inline-code-attrs-start title = 'save_comments_bulk の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import SaveCommentsBulkOptions
from client.models.create_comment_params import CreateCommentParams
from client.models.save_comments_bulk_response import SaveCommentsBulkResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# configuration.py でサポートされているすべての設定パラメータの一覧を確認できます
# クライアントは認証および認可パラメータを設定する必要があります
# APIサーバーのセキュリティポリシーに従って
# 各認証方式の例が以下に提供されているので、使用ケースに合致する例を使用してください
# あなたの認証使用ケースを満たすものを

# APIキー認証を設定: api_key
# 必要に応じて、APIキーのプレフィックス（例: Bearer）を設定するには、以下のコメントを解除してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# APIクライアントのインスタンスでコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # APIクラスのインスタンスを作成
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_comment_params = [client.CreateCommentParams()] # List[CreateCommentParams] | 
    is_live = True # bool |  (オプション)
    do_spam_check = True # bool |  (オプション)
    send_emails = True # bool |  (オプション)
    populate_notifications = True # bool |  (オプション)

    try:
        api_response = api_instance.save_comments_bulk(tenant_id, create_comment_params, SaveCommentsBulkOptions(is_live=is_live, do_spam_check=do_spam_check, send_emails=send_emails, populate_notifications=populate_notifications))
        print("The response of DefaultApi->save_comments_bulk:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->save_comments_bulk: %s\n" % e)
[inline-code-end]