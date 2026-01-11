Загрузить и изменить размер изображения

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| sizePreset | string | query | Нет | Пресет размера: "Default" (1000x1000px) или "CrossPlatform" (создаёт размеры для популярных устройств) |
| urlId | string | query | Нет | id страницы, с которой происходит загрузка, для настройки |

## Ответ

Возвращает: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/upload_image_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример upload_image'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
file = File.new('/path/to/some/file') # File | 
opts = {
  size_preset: FastCommentsClient::SizePreset::DEFAULT, # SizePreset | Пресет размера: \"Default\" (1000x1000px) или \"CrossPlatform\" (создаёт размеры для популярных устройств)
  url_id: 'url_id_example' # String | id страницы, с которой происходит загрузка, для настройки
}

begin
  
  result = api_instance.upload_image(tenant_id, file, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->upload_image: #{e}"
end
[inline-code-end]