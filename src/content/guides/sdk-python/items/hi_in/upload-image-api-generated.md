एक इमेज अपलोड और रीसाइज़ करें

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| sizePreset | string | query | नहीं | साइज़ प्रीसेट: "Default" (1000x1000px) या "CrossPlatform" (लोकप्रिय डिवाइसों के लिए साइज़ बनाता है) |
| urlId | string | query | नहीं | वह पेज id जिससे अपलोड हो रहा है, कॉन्फ़िगर करने के लिए |

## प्रतिक्रिया

रिटर्न करता है: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/upload_image_response.py)

## उदाहरण

[inline-code-attrs-start title = 'upload_image उदाहरण'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import UploadImageOptions
from client.models.size_preset import SizePreset
from client.models.upload_image_response import UploadImageResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    file = None # bytes | 
    size_preset = client.SizePreset() # SizePreset | साइज़ प्रीसेट: \"Default\" (1000x1000px) या \"CrossPlatform\" (लोकप्रिय डिवाइसों के लिए साइज़ बनाता है) (optional)
    url_id = 'url_id_example' # str | वह पेज id जिससे अपलोड हो रहा है, कॉन्फ़िगर करने के लिए (optional)

    try:
        api_response = api_instance.upload_image(tenant_id, file, UploadImageOptions(size_preset=size_preset, url_id=url_id))
        print("The response of PublicApi->upload_image:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->upload_image: %s\n" % e)
[inline-code-end]