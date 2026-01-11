Качване и преоразмеряване на изображение

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| sizePreset | string | query | Не | Преднастройка на размера: "Default" (1000x1000px) или "CrossPlatform" (създава размери за популярни устройства) |
| urlId | string | query | Не | ID на страницата, от която се извършва качването, за конфигуриране |

## Отговор

Връща: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/upload_image_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример за upload_image'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
file = File.new('/path/to/some/file') # File | 
opts = {
  size_preset: FastCommentsClient::SizePreset::DEFAULT, # SizePreset | Преднастройка на размера: \"Default\" (1000x1000px) или \"CrossPlatform\" (създава размери за популярни устройства)
  url_id: 'url_id_example' # String | ID на страницата, от която се извършва качването, за конфигуриране
}

begin
  
  result = api_instance.upload_image(tenant_id, file, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->upload_image: #{e}"
end
[inline-code-end]

---