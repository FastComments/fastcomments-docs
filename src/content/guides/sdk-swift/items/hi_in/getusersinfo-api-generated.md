किसी टेनेंट के लिए सामूहिक उपयोगकर्ता जानकारी। दिए गए userIds के लिए, User / SSOUser से डिस्प्ले जानकारी लौटाएँ।
कमेंट विजेट द्वारा उपयोग किया जाता है ताकि प्रेज़ेंस इवेंट के माध्यम से अभी दिखाई देने वाले उपयोगकर्ताओं को समृद्ध (enrich) किया जा सके।
कोई पेज संदर्भ नहीं: गोपनीयता समान रूप से लागू की जाती है (निजी प्रोफ़ाइल मास्क कर दी जाती हैं)।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| ids | string | query | हाँ | कॉमा से अलग किए गए userIds। |

## प्रतिक्रिया

लौटाता है: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersInfoResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getUsersInfo उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी बीटा हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new पर रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let ids = "ids_example" // String | कॉमा से अलग किए गए userIds।

PublicAPI.getUsersInfo(tenantId: tenantId, ids: ids) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]