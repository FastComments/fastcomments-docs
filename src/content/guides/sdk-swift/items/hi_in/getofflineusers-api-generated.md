पेज पर उन पिछले टिप्पणीकारों (commenters) की सूची जो इस समय ऑनलाइन नहीं हैं। displayName के अनुसार क्रमबद्ध।
"Members" सेक्शन रेंडर करने के लिए /users/online को पूरा करने के बाद इसका उपयोग करें।
commenterName पर कर्सर पेजिनेशन: सर्वर आंशिक {tenantId, urlId, commenterName}
इंडेक्स को afterName से $gt के माध्यम से आगे बढ़ाता है, कोई $skip लागत नहीं।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ | पेज URL पहचानकर्ता (सर्वर-साइड पर साफ़ किया गया)। |
| afterName | string | query | नहीं | कर्सर: पिछली प्रतिक्रिया से nextAfterName पास करें। |
| afterUserId | string | query | नहीं | कर्सर टाइब्रेकर: पिछली प्रतिक्रिया से nextAfterUserId पास करें। afterName सेट होने पर आवश्यक ताकि समान नाम वाली प्रविष्टियाँ छूट न जाएँ। |

## प्रतिक्रिया

रिटर्न करता है: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getOfflineUsers उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | पेज URL पहचानकर्ता (सर्वर-साइड पर साफ़ किया गया)।
let afterName = "afterName_example" // String | कर्सर: पिछली प्रतिक्रिया से nextAfterName पास करें। (optional)
let afterUserId = "afterUserId_example" // String | कर्सर टाइब्रेकर: पिछली प्रतिक्रिया से nextAfterUserId पास करें। afterName सेट होने पर आवश्यक ताकि समान नाम वाली प्रविष्टियाँ छूट न जाएँ। (optional)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]