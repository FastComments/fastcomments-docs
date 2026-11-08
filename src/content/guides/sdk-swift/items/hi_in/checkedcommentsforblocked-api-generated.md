## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| commentIds | string | query | हाँ | कॉमा से अलग की गई कमेंट आईडी की सूची। |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CheckBlockedCommentsResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'checkedCommentsForBlocked उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा में हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new पर रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentIds = "commentIds_example" // String | कॉमा से अलग की गई कमेंट आईडी की सूची।
let sso = "sso_example" // String |  (वैकल्पिक)

PublicAPI.checkedCommentsForBlocked(tenantId: tenantId, commentIds: commentIds, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]