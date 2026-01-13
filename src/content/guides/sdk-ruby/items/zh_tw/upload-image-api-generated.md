---
上傳並調整圖片大小

## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| sizePreset | string | query | 否 | 大小預設： "Default" (1000x1000px) 或 "CrossPlatform" (為熱門裝置建立尺寸) |
| urlId | string | query | 否 | 上傳所屬的頁面 ID，用於設定 |

## 回應

回傳：[`UploadImageResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/upload_image_response.rb)

## 範例

[inline-code-attrs-start title = 'upload_image 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
file = File.new('/path/to/some/file') # File | 
opts = {
  size_preset: FastCommentsClient::SizePreset::DEFAULT, # SizePreset | 大小預設： \"Default\" (1000x1000px) 或 \"CrossPlatform\" (為熱門裝置建立尺寸)
  url_id: 'url_id_example' # String | 上傳所屬的頁面 ID，用於設定
}

begin
  
  result = api_instance.upload_image(tenant_id, file, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->upload_image: #{e}"
end
[inline-code-end]

---