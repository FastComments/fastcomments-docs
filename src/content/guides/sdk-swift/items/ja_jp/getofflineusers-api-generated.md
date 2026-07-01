Past commenters on the page who are NOT currently online. Sorted by displayName.  
Use this after exhausting /users/online to render a "Members" section.  
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| urlId | string | query | はい | ページ URL 識別子（サーバー側でクリーンアップ済み）。 |
| afterName | string | query | いいえ | カーソル: 前回のレスポンスから nextAfterName を渡します。 |
| afterUserId | string | query | いいえ | カーソルのタイブレーカー: 前回のレスポンスから nextAfterUserId を渡します。afterName が設定されている場合に必要で、名前が同じエントリが除外されないようにします。 |

## レスポンス

返却: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## 例

[inline-code-attrs-start title = 'getOfflineUsers の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題があれば、http://github.com/OpenAPITools/openapi-generator/issues/new へ報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | ページ URL 識別子（サーバー側でクリーンアップ済み）。 
let afterName = "afterName_example" // String | カーソル: 前回のレスポンスから nextAfterName を渡します。（任意）
let afterUserId = "afterUserId_example" // String | カーソルのタイブレーカー: 前回のレスポンスから nextAfterUserId を渡します。afterName が設定されている場合に必要で、名前が同じエントリが除外されないようにします。（任意）

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