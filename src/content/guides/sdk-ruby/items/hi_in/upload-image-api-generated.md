एक इमेज अपलोड और रिसाइज़ करें

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| sizePreset | string | query | नहीं | साइज़ प्रीसेट: "Default" (1000x1000px) या "CrossPlatform" (लोकप्रिय डिवाइसों के लिए साइज़ बनाता है) |
| urlId | string | query | नहीं | वह पेज id जिससे अपलोड हो रहा है, कॉन्फ़िगर करने के लिए |

## प्रतिक्रिया

लौटाता है: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/upload_image_response.rb)

## उदाहरण

[inline-code-attrs-start title = 'upload_image उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
file = File.new('/path/to/some/file') # File | 
opts = {
  size_preset: FastCommentsClient::SizePreset::DEFAULT, # SizePreset | साइज़ प्रीसेट: \"Default\" (1000x1000px) या \"CrossPlatform\" (लोकप्रिय डिवाइसों के लिए साइज़ बनाता है)
  url_id: 'url_id_example' # String | वह पेज id जिससे अपलोड हो रहा है, कॉन्फ़िगर करने के लिए
}

begin
  
  result = api_instance.upload_image(tenant_id, file, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->upload_image: #{e}"
end
[inline-code-end]