---
Webhook 配信が使用する正確な形状の最近のコメントで、統合（例：Zapier のサンプルデータ）を構築するために使用します。すべてのイベントは同じコメントオブジェクトを配信するため、`event` は有効である必要があります。

## Parameters

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| event | string | query | いいえ |  |
| limit | number | query | いいえ |  |

## Response

返却: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetWebhookSamplePayloadsResponse.swift)

## Example

[inline-code-attrs-start title = 'getWebhookSamplePayloads の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は、http://github.com/OpenAPITools/openapi-generator/issues/new へ報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // 文字列 | 
let event = WebhookEventName() // WebhookEventName |  (オプション)
let limit = 987 // Double |  (オプション)

DefaultAPI.getWebhookSamplePayloads(tenantId: tenantId, options: DefaultAPI.GetWebhookSamplePayloadsOptions(event: event, limit: limit)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---