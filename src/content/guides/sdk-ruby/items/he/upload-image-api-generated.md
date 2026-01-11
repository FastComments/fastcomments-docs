העלאה ושינוי גודל תמונה

## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| sizePreset | string | query | לא | ערכת גודל: "Default" (1000x1000px) או "CrossPlatform" (יוצר גדלים למכשירים פופולריים) |
| urlId | string | query | לא | מזהה העמוד שממנו מתבצעת ההעלאה, לצורך קונפיגורציה |

## תגובה

מחזיר: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/upload_image_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-upload_image'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
file = File.new('/path/to/some/file') # File | 
opts = {
  size_preset: FastCommentsClient::SizePreset::DEFAULT, # SizePreset | ערכת גודל: \"Default\" (1000x1000px) או \"CrossPlatform\" (יוצר גדלים למכשירים פופולריים)
  url_id: 'url_id_example' # String | מזהה העמוד שממנו מתבצעת ההעלאה, לצורך קונפיגורציה
}

begin
  
  result = api_instance.upload_image(tenant_id, file, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->upload_image: #{e}"
end
[inline-code-end]