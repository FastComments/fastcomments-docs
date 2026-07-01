画像のアップロードとリサイズ

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | サイズプリセット: "Default" (1000x1000px) または "CrossPlatform" (一般的なデバイス用のサイズを作成) |
| urlId | string | query | No | アップロードが行われているページの ID、設定用 |

## レスポンス

返却: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## 例

[inline-code-attrs-start title = 'uploadImage の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は、http://github.com/OpenAPITools/openapi-generator/issues/new へ報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | サイズプリセット: \"Default\" (1000x1000px) または \"CrossPlatform\" (一般的なデバイス用のサイズを作成) （オプション） 
let urlId = "urlId_example" // String | アップロードが行われているページの ID、設定用 （オプション）

PublicAPI.uploadImage(tenantId: tenantId, file: file, options: PublicAPI.UploadImageOptions(sizePreset: sizePreset, urlId: urlId)) { (response, error) in
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