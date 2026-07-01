---
上传并调整图像大小

## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | 尺寸预设：\"Default\" (1000x1000px) 或 \"CrossPlatform\" (为流行设备创建尺寸) |
| urlId | string | query | No | 上传所在页面的 ID，用于配置 |

## 响应

Returns: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## 示例

[inline-code-attrs-start title = 'uploadImage 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于 beta 阶段。如有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | 尺寸预设：\"Default\" (1000x1000px) 或 \"CrossPlatform\" (为流行设备创建尺寸) (可选)
let urlId = "urlId_example" // String | 上传所在页面的 ID，用于配置 (可选)

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