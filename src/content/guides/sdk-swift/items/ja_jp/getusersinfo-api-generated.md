---
テナントの複数ユーザー情報。userIds を受け取り、User / SSOUser から表示情報を返します。
コメントウィジェットが、プレゼンスイベントで新たに現れたユーザーの情報を補完するために使用します。
ページコンテキストがないため、プライバシーは一律に適用されます（非公開プロファイルはマスクされます）。

## パラメータ

| 名前 | 型 | 位置 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | カンマ区切りの userIds。 |

## レスポンス

返却: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersInfoResponse.swift)

## 例

[inline-code-attrs-start title = 'getUsersInfo の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new から報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let ids = "ids_example" // String | カンマ区切りの userIds.

PublicAPI.getUsersInfo(tenantId: tenantId, ids: ids) { (response, error) in
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