किसी टेनेंट के लिए पेजों की सूची बनाता है। FChat डेस्कटॉप क्लाइंट द्वारा अपनी रूम सूची भरने के लिए उपयोग किया जाता है।
प्रत्येक पेज के लिए resolved custom config पर `enableFChat` true होना आवश्यक है।
SSO की आवश्यकता वाले पेज अनुरोध करने वाले उपयोगकर्ता के group access के आधार पर फ़िल्टर किए जाते हैं।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| cursor | string | query | नहीं | पिछले अनुरोध से `nextCursor` के रूप में लौटाया गया अपारदर्शी पेजिनेशन कर्सर। उसी `sortBy` से बंधा हुआ। |
| limit | integer | query | नहीं | 1..200, डिफ़ॉल्ट 50 |
| q | string | query | नहीं | वैकल्पिक, केस-असंवेदनशील शीर्षक प्रीफ़िक्स फ़िल्टर। |
| sortBy | string | query | नहीं | सॉर्ट क्रम। `updatedAt` (डिफ़ॉल्ट, नवीनतम पहले), `commentCount` (सबसे अधिक टिप्पणियाँ पहले), या `title` (वर्णानुक्रमिक)। |
| hasComments | boolean | query | नहीं | यदि true है, तो केवल कम से कम एक टिप्पणी वाले पेज लौटाएं। |

## प्रतिक्रिया

लौटाता है: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getPagesPublic उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | पिछले अनुरोध से `nextCursor` के रूप में लौटाया गया अपारदर्शी पेजिनेशन कर्सर। उसी `sortBy` से बंधा हुआ। (optional)
let limit = 987 // Int | 1..200, डिफ़ॉल्ट 50 (optional)
let q = "q_example" // String | वैकल्पिक, केस-असंवेदनशील शीर्षक प्रीफ़िक्स फ़िल्टर। (optional)
let sortBy = PagesSortBy() // PagesSortBy | सॉर्ट क्रम। `updatedAt` (डिफ़ॉल्ट, नवीनतम पहले), `commentCount` (सबसे अधिक टिप्पणियाँ पहले), या `title` (वर्णानुक्रमिक)। (optional)
let hasComments = true // Bool | यदि true है, तो केवल कम से कम एक टिप्पणी वाले पेज लौटाएं। (optional)

PublicAPI.getPagesPublic(tenantId: tenantId, options: PublicAPI.GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]