Carica e ridimensiona un'immagine

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Preset dimensioni: "Default" (1000x1000px) o "CrossPlatform" (crea dimensioni per dispositivi popolari) |
| urlId | string | query | No | ID della pagina da cui avviene il caricamento, per configurare |

## Risposta

Restituisce: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/upload_image_response.rb)

## Esempio

[inline-code-attrs-start title = 'Esempio di upload_image'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
file = File.new('/path/to/some/file') # File | 
opts = {
  size_preset: FastCommentsClient::SizePreset::DEFAULT, # SizePreset | Preset dimensioni: \"Default\" (1000x1000px) o \"CrossPlatform\" (crea dimensioni per dispositivi popolari)
  url_id: 'url_id_example' # String | ID della pagina da cui avviene il caricamento, per configurare
}

begin
  
  result = api_instance.upload_image(tenant_id, file, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->upload_image: #{e}"
end
[inline-code-end]