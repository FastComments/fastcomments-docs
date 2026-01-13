上傳並調整圖片大小

## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | 大小預設： "Default" (1000x1000px) 或 "CrossPlatform" (為常用裝置產生尺寸) |
| urlId | string | query | No | 上傳發生的頁面 ID，用以設定 |

## 回應

回傳: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## 範例

[inline-code-attrs-start title = 'uploadImage 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 下列程式範例仍為測試版。如有問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | 大小預設：\"Default\" (1000x1000px) 或 \"CrossPlatform\" (為常用裝置產生尺寸) (選填)
let urlId = "urlId_example" // String | 上傳發生的頁面 ID，用以設定 (選填)

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