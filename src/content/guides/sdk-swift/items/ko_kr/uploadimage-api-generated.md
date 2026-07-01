이미지를 업로드하고 크기를 조정합니다

## Parameters

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | 크기 프리셋: "Default" (1000x1000px) 또는 "CrossPlatform" (인기 기기용 사이즈 생성) |
| urlId | string | query | No | 업로드가 발생하는 페이지 ID, 구성용 |

## Response

Returns: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Example

[inline-code-attrs-start title = 'uploadImage 예시'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타 버전입니다. 문제가 있으면 http://github.com/OpenAPITools/openapi-generator/issues/new 에 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | 크기 프리셋: \"Default\" (1000x1000px) 또는 \"CrossPlatform\" (인기 기기용 사이즈 생성) (optional)
let urlId = "urlId_example" // String | 업로드가 발생하는 페이지 ID, 구성용 (optional)

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