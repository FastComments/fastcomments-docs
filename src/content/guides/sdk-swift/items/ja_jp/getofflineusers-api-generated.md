ページ上の、現在オンラインではない過去のコメント投稿者。displayName でソートされています。
/users/online を使い切った後に、「メンバー」セクションを表示するために使用します。
commenterName によるカーソルページネーション: サーバーは部分的な {tenantId, urlId, commenterName} インデックスを afterName から $gt 経由で前方に走査します。$skip のコストはかかりません。

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| urlId | string | query | はい | ページのURL識別子（サーバー側でクリーンされます）。 |
| afterName | string | query | いいえ | カーソル: 前回のレスポンスの nextAfterName を渡します。 |
| afterUserId | string | query | いいえ | カーソルのタイブレーカー: 前回のレスポンスの nextAfterUserId を渡します。afterName が設定されている場合は、名前が同じエントリが落ちないようにこれが必須です。 |

## レスポンス

戻り値: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## 例

[inline-code-attrs-start title = 'getOfflineUsers の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | ページのURL識別子（サーバー側でクリーンされます）。
let afterName = "afterName_example" // String | カーソル: 前回のレスポンスの nextAfterName を渡します。 (オプション)
let afterUserId = "afterUserId_example" // String | カーソルのタイブレーカー: 前回のレスポンスの nextAfterUserId を渡します。afterName が設定されている場合は、名前が同じエントリが落ちないように必須です。 (オプション)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]