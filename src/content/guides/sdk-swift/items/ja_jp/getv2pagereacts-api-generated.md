## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| urlId | string | query | はい |  |
| sso | string | query | いいえ |  |
| includeUserInfo | string | query | いいえ | 設定すると、各リアクト ID のユーザー（アバターとユーザー名）を返します。\"public\" のみが受け入れられ、プライベートプロファイルは省略されます。 |

## 応答

返却: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetV2PageReacts.swift)

## 例

[inline-code-attrs-start title = 'getV2PageReacts の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は、http://github.com/OpenAPITools/openapi-generator/issues/new へ報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (オプション)
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | 設定すると、各リアクト ID のユーザー（アバターとユーザー名）を返します。\"public\" のみが受け入れられ、プライベートプロファイルは省略されます。 (オプション)

PublicAPI.getV2PageReacts(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetV2PageReactsOptions(sso: sso, includeUserInfo: includeUserInfo)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]