テナントに設定されたwebhookの一覧を返します。ダッシュボードで管理されている行とAPIサブスクリプションの両方が対象です。

## パラメーター

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| event | string | query | いいえ |  |
| domain | string | query | いいえ |  |
| source | string | query | いいえ |  |
| skip | number | query | いいえ |  |

## レスポンス

戻り値: [`GetWebhooksResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetWebhooksResponse.swift)

## 例

[inline-code-attrs-start title = 'getWebhooks の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題があれば http://github.com/OpenAPITools/openapi-generator/issues/new まで報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let event = WebhookEventName() // WebhookEventName |  (optional)
let domain = "domain_example" // String |  (optional)
let source = WebhookSource() // WebhookSource |  (optional)
let skip = 987 // Double |  (optional)

DefaultAPI.getWebhooks(tenantId: tenantId, options: DefaultAPI.GetWebhooksOptions(event: event, domain: domain, source: source, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]