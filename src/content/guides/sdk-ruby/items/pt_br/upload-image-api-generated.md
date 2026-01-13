Enviar e redimensionar uma imagem

## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sim |  |
| sizePreset | string | query | Não | Predefinição de tamanho: "Default" (1000x1000px) ou "CrossPlatform" (cria tamanhos para dispositivos populares) |
| urlId | string | query | Não | ID da página de onde o upload está ocorrendo, para configurar |

## Resposta

Retorna: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/upload_image_response.rb)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de upload_image'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
file = File.new('/path/to/some/file') # File | 
opts = {
  size_preset: FastCommentsClient::SizePreset::DEFAULT, # SizePreset | Predefinição de tamanho: \"Default\" (1000x1000px) ou \"CrossPlatform\" (cria tamanhos para dispositivos populares)
  url_id: 'url_id_example' # String | ID da página de onde o upload está ocorrendo, para configurar
}

begin
  
  result = api_instance.upload_image(tenant_id, file, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->upload_image: #{e}"
end
[inline-code-end]

---