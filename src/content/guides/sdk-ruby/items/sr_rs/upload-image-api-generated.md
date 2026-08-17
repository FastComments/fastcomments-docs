Upload and resize an image

## Parameters

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Претподешавање величине: "Default" (1000x1000px) или "CrossPlatform" (прави величине за популарне уређаје) |
| urlId | string | query | No | Идентификатор странице са које се врши отпремање, за подешавање |

## Response

Враћа: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/upload_image_response.rb)

## Example

[inline-code-attrs-start title = 'Пример за upload_image'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
file = File.new('/path/to/some/file') # File | 
opts = {
  size_preset: FastCommentsClient::SizePreset::DEFAULT, # SizePreset | Претподешавање величине: "Default" (1000x1000px) или "CrossPlatform" (прави величине за популарне уређаје)
  url_id: 'url_id_example' # String | Идентификатор странице са које се врши отпремање, за подешавање
}

begin
  
  result = api_instance.upload_image(tenant_id, file, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->upload_image: #{e}"
end
[inline-code-end]