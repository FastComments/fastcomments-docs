이미지 업로드 및 크기 조정

## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| sizePreset | string | query | 아니오 | 크기 프리셋: "Default" (1000x1000px) 또는 "CrossPlatform" (인기 있는 기기용 크기를 생성합니다) |
| urlId | string | query | 아니오 | 업로드가 발생하는 페이지 ID(구성용) |

## 응답

반환: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/upload_image_response.rb)

## 예제

[inline-code-attrs-start title = 'upload_image 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
file = File.new('/path/to/some/file') # File | 
opts = {
  size_preset: FastCommentsClient::SizePreset::DEFAULT, # SizePreset | 크기 프리셋: \"Default\" (1000x1000px) 또는 \"CrossPlatform\" (인기 있는 기기용 크기를 생성합니다)
  url_id: 'url_id_example' # String | 업로드가 발생하는 페이지 ID(구성용)
}

begin
  
  result = api_instance.upload_image(tenant_id, file, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->upload_image: #{e}"
end
[inline-code-end]