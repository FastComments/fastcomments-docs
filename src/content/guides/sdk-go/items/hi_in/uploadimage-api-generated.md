एक इमेज अपलोड और रिसाइज़ करें

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| sizePreset | string | query | नहीं | साइज़ प्रीसेट: "Default" (1000x1000px) या "CrossPlatform" (लोकप्रिय डिवाइसों के लिए साइज़ बनाता है) |
| urlId | string | query | नहीं | वह पेज id जिससे अपलोड हो रहा है, कॉन्फ़िगर करने के लिए |

## प्रतिक्रिया

रिटर्न करता है: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_upload_image_response.go)

## उदाहरण

[inline-code-attrs-start title = 'UploadImage उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	file := os.NewFile(1234, "some_file") // *os.File | 
	sizePreset := openapiclient.SizePreset("Default") // SizePreset | साइज़ प्रीसेट: \"Default\" (1000x1000px) या \"CrossPlatform\" (लोकप्रिय डिवाइसों के लिए साइज़ बनाता है) (optional)
	urlId := "urlId_example" // string | वह पेज id जिससे अपलोड हो रहा है, कॉन्फ़िगर करने के लिए (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.UploadImage(context.Background(), tenantId).File(file).SizePreset(sizePreset).UrlId(urlId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.UploadImage``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `UploadImage`: UploadImageResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.UploadImage`: %v\n", resp)
}
[inline-code-end]