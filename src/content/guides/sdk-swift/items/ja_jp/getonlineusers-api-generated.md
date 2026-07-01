Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.  
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

## Parameters

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | ページ URL 識別子（サーバー側でクリーンアップ済み）。 |
| afterName | string | query | No | カーソル: 前回のレスポンスから nextAfterName を渡す。 |
| afterUserId | string | query | No | カーソルのタイブレーカー: 前回のレスポンスから nextAfterUserId を渡す。afterName が設定されている場合に必須で、名前が同じ場合にエントリが除外されません。 |

## Response

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## 例

[inline-code-attrs-start title = 'getOnlineUsers の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は、http://github.com/OpenAPITools/openapi-generator/issues/new へ報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | ページ URL 識別子（サーバー側でクリーンアップ済み）。
let afterName = "afterName_example" // String | カーソル: 前回のレスポンスから nextAfterName を渡す。(optional)
let afterUserId = "afterUserId_example" // String | カーソルのタイブレーカー: 前回のレスポンスから nextAfterUserId を渡す。afterName が設定されている場合に必須で、名前が同じ場合にエントリが除外されません。(optional)

PublicAPI.getOnlineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]