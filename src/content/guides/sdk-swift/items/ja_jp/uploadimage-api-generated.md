画像をアップロードしてリサイズする

## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| sizePreset | string | query | いいえ | サイズプリセット: "Default" (1000x1000px) または "CrossPlatform" (一般的なデバイス向けのサイズを作成します) |
| urlId | string | query | いいえ | アップロード元のページID（設定用） |

## レスポンス

戻り値: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## 例

[inline-code-attrs-start title = 'uploadImage の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | サイズプリセット: \"Default\" (1000x1000px) または \"CrossPlatform\" (一般的なデバイス向けのサイズを作成します) (オプション)
let urlId = "urlId_example" // String | アップロード元のページID（設定用） (オプション)

PublicAPI.uploadImage(tenantId: tenantId, file: file, sizePreset: sizePreset, urlId: urlId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]