Отпреми и промијени величину слике

## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Пресет величине: "Default" (1000x1000px) или "CrossPlatform" (ствара величине за популарне уређаје) |
| urlId | string | query | No | ID странице са које се врши отпремање, за конфигурацију |

## Одговор

Враћа: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/upload_image_response.rb)

## Пример

[inline-code-attrs-start title = 'upload_image Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
file = File.new('/path/to/some/file') # File | 
opts = {
  size_preset: FastCommentsClient::SizePreset::DEFAULT, # SizePreset | Пресет величине: \"Default\" (1000x1000px) или \"CrossPlatform\" (ствара величине за популарне уређаје)
  url_id: 'url_id_example' # String | ID странице са које се врши отпремање, за конфигурацију
}

begin
  
  result = api_instance.upload_image(tenant_id, file, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->upload_image: #{e}"
end
[inline-code-end]