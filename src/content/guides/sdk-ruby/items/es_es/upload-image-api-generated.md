Subir y redimensionar una imagen

## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| sizePreset | string | query | No | Preajuste de tamaño: "Default" (1000x1000px) o "CrossPlatform" (genera tamaños para dispositivos populares) |
| urlId | string | query | No | ID de página desde la que se realiza la subida, para configurar |

## Respuesta

Devuelve: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/upload_image_response.rb)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de upload_image'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
file = File.new('/path/to/some/file') # File | 
opts = {
  size_preset: FastCommentsClient::SizePreset::DEFAULT, # SizePreset | Preajuste de tamaño: \"Default\" (1000x1000px) o \"CrossPlatform\" (genera tamaños para dispositivos populares)
  url_id: 'url_id_example' # String | ID de página desde la que se realiza la subida, para configurar
}

begin
  
  result = api_instance.upload_image(tenant_id, file, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->upload_image: #{e}"
end
[inline-code-end]