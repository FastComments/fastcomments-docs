---
現在ページにオンラインの閲覧者: 現在そのページに websocket セッションがページにサブスクライブされている人々。
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| urlId | string | query | はい | ページのURL識別子（サーバー側でクリーン処理済み）。 |
| afterName | string | query | いいえ | カーソル：前回のレスポンスの nextAfterName を渡します。 |
| afterUserId | string | query | いいえ | カーソルのタイブレーカー：前回のレスポンスの nextAfterUserId を渡します。afterName が設定されている場合、同名の競合でエントリが落ちないように必須です。 |

## レスポンス

返却: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## 例

[inline-code-attrs-start title = 'getOnlineUsers の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | ページのURL識別子（サーバー側でクリーン処理済み）。
let afterName = "afterName_example" // String | カーソル：前回のレスポンスの nextAfterName を渡します。（省略可）
let afterUserId = "afterUserId_example" // String | カーソルのタイブレーカー：前回のレスポンスの nextAfterUserId を渡します。afterName が設定されている場合、同名の競合でエントリが落ちないように必須です。（省略可）

PublicAPI.getOnlineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
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