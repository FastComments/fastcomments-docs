Bild hochladen und skalieren

## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| sizePreset | string | query | Nein | Größen-Voreinstellung: "Default" (1000x1000px) oder "CrossPlatform" (erstellt Größen für beliebte Geräte) |
| urlId | string | query | Nein | Seiten-ID, von der das Hochladen erfolgt, zur Konfiguration |

## Antwort

Rückgabe: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/upload_image_response.rb)

## Beispiel

[inline-code-attrs-start title = 'upload_image Beispiel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
file = File.new('/path/to/some/file') # File | 
opts = {
  size_preset: FastCommentsClient::SizePreset::DEFAULT, # SizePreset | Größen-Voreinstellung: \"Default\" (1000x1000px) oder \"CrossPlatform\" (erstellt Größen für beliebte Geräte)
  url_id: 'url_id_example' # String | Seiten-ID, von der das Hochladen erfolgt, zur Konfiguration
}

begin
  
  result = api_instance.upload_image(tenant_id, file, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->upload_image: #{e}"
end
[inline-code-end]