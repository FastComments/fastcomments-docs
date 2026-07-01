上傳並調整圖像大小

## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| sizePreset | string | query | 否 | 尺寸預設值: "Default" (1000x1000px) 或 "CrossPlatform" (為流行裝置建立多種尺寸) |
| urlId | string | query | 否 | 上傳發生的頁面 ID，用於設定（可選） |

## 回應

回傳：[`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## 範例

[inline-code-attrs-start title = 'uploadImage 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下程式碼範例仍屬測試版。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | 尺寸預設值: \"Default\" (1000x1000px) 或 \"CrossPlatform\" (為流行裝置建立多種尺寸) (optional)
let urlId = "urlId_example" // String | 上傳發生的頁面 ID，用於設定（optional）

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