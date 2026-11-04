URLをコメントイベントにサブスクライブします(REST hook subscribe)。同じURLを同じイベントとドメインに再度サブスクライブした場合は、既存のサブスクリプションが返されます。配信はHMACで署名されます。詳細はwebhooksガイドを参照してください。レガシーな `token` ヘッダーはAPIサブスクリプションには送信されません。

## パラメーター

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |

## レスポンス

戻り値: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateWebhookResponse.swift)

## 例

[inline-code-attrs-start title = 'createWebhook の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題が発生した場合は、http://github.com/OpenAPITools/openapi-generator/issues/new から報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createWebhookParams = CreateWebhookParams(url: "url_example", event: WebhookEventName(), domain: "domain_example", method: WebhookHTTPMethod()) // CreateWebhookParams | 

DefaultAPI.createWebhook(tenantId: tenantId, createWebhookParams: createWebhookParams) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]