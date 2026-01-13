上传并调整图像大小

## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| sizePreset | string | query | 否 | 大小预设： "Default"（1000x1000px）或 "CrossPlatform"（为流行设备创建尺寸） |
| urlId | string | query | 否 | 上传发生的页面 id，用于配置 |

## 响应

返回: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/upload_image_response.rb)

## 示例

[inline-code-attrs-start title = 'upload_image 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
file = File.new('/path/to/some/file') # File | 
opts = {
  size_preset: FastCommentsClient::SizePreset::DEFAULT, # SizePreset | 大小预设：\"Default\"（1000x1000px）或 \"CrossPlatform\"（为流行设备创建尺寸）
  url_id: 'url_id_example' # String | 上传发生的 Page id，用于配置
}

begin
  
  result = api_instance.upload_image(tenant_id, file, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->upload_image: #{e}"
end
[inline-code-end]