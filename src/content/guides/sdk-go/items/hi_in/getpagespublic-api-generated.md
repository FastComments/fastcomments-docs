किसी टेनेंट के लिए पेजों की सूची लौटाता है। FChat डेस्कटॉप क्लाइंट द्वारा अपनी रूम सूची भरने के लिए उपयोग किया जाता है।
प्रत्येक पेज के लिए resolved custom config पर `enableFChat` true होना आवश्यक है।
जो पेज SSO की मांग करते हैं, उन्हें अनुरोध करने वाले उपयोगकर्ता के group access के विरुद्ध फ़िल्टर किया जाता है।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| cursor | string | query | नहीं | पिछले अनुरोध से `nextCursor` के रूप में लौटाया गया अपारदर्शी पेजिनेशन कर्सर। उसी `sortBy` से बंधा हुआ। |
| limit | integer | query | नहीं | 1..200, डिफ़ॉल्ट 50 |
| q | string | query | नहीं | वैकल्पिक केस-असंवेदनशील शीर्षक प्रीफ़िक्स फ़िल्टर। |
| sortBy | string | query | नहीं | क्रमबद्ध क्रम। `updatedAt` (डिफ़ॉल्ट, नवीनतम पहले), `commentCount` (सर्वाधिक टिप्पणियां पहले), या `title` (वर्णानुक्रमिक)। |
| hasComments | boolean | query | नहीं | यदि true, तो केवल कम से कम एक टिप्पणी वाले पेज लौटाएं। |

## प्रतिक्रिया

लौटाता है: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_public_pages_response.go)

## उदाहरण

[inline-code-attrs-start title = 'GetPagesPublic उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	cursor := "cursor_example" // string | पिछले अनुरोध से `nextCursor` के रूप में लौटाया गया अपारदर्शी पेजिनेशन कर्सर। उसी `sortBy` से बंधा हुआ। (optional)
	limit := int32(56) // int32 | 1..200, डिफ़ॉल्ट 50 (optional)
	q := "q_example" // string | वैकल्पिक केस-असंवेदनशील शीर्षक प्रीफ़िक्स फ़िल्टर। (optional)
	sortBy := openapiclient.PagesSortBy("updatedAt") // PagesSortBy | क्रमबद्ध क्रम। `updatedAt` (डिफ़ॉल्ट, नवीनतम पहले), `commentCount` (सर्वाधिक टिप्पणियां पहले), या `title` (वर्णानुक्रमिक)। (optional)
	hasComments := true // bool | यदि true, तो केवल कम से कम एक टिप्पणी वाले पेज लौटाएं। (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetPagesPublic(context.Background(), tenantId).Cursor(cursor).Limit(limit).Q(q).SortBy(sortBy).HasComments(hasComments).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetPagesPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetPagesPublic`: GetPublicPagesResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetPagesPublic`: %v\n", resp)
}
[inline-code-end]