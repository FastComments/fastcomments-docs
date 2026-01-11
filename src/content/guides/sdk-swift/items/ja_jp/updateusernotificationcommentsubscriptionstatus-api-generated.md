特定のコメントに対する通知を有効または無効にします。

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| notificationId | string | path | はい |  |
| optedInOrOut | string | path | はい |  |
| commentId | string | query | はい |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UpdateUserNotificationStatus200Response.swift)

## 例

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 次のコードサンプルはまだベータ版です。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new を通じて報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let notificationId = "notificationId_example" // String | 
let optedInOrOut = "optedInOrOut_example" // String | 
let commentId = "commentId_example" // String | 
let sso = "sso_example" // String |  (オプション)

PublicAPI.updateUserNotificationCommentSubscriptionStatus(tenantId: tenantId, notificationId: notificationId, optedInOrOut: optedInOrOut, commentId: commentId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]