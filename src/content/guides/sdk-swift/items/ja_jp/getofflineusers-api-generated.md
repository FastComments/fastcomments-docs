現在オンラインでない、そのページの過去のコメント投稿者。displayName でソートされます。
/users/online を使い切った後にこの API を使用して「メンバー」セクションを表示します。
commenterName によるカーソルページネーション: サーバーは部分インデックス {tenantId, urlId, commenterName} を afterName から $gt を使って順に走査するため、$skip のコストはかかりません。

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| urlId | string | query | はい | ページ URL 識別子(サーバー側でクリーンアップされます)。 |
| afterName | string | query | いいえ | カーソル: 前のレスポンスの nextAfterName を渡します。 |
| afterUserId | string | query | いいえ | カーソルのタイブレーカー: 前のレスポンスの nextAfterUserId を渡します。afterName を設定した場合は、同一名によるエントリの欠落を防ぐため必須です。 |

## レスポンス

戻り値: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## 例

[inline-code-attrs-start title = 'getOfflineUsers の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | ページ URL 識別子(サーバー側でクリーンアップされます)。
let afterName = "afterName_example" // String | カーソル: 前のレスポンスの nextAfterName を渡します。 (optional)
let afterUserId = "afterUserId_example" // String | カーソルのタイブレーカー: 前のレスポンスの nextAfterUserId を渡します。afterName を設定した場合は、同一名によるエントリの欠落を防ぐため必須です。 (optional)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]