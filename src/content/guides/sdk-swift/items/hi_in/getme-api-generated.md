उपयोग में आने वाले क्रेडेंशियल की पहचान करता है: वह टेनेंट जिसका यह हिस्सा है और, OAuth टोकन के लिए, वह उपयोगकर्ता जिसने इसे अधिकृत किया।
इंटीग्रेशन इसका उपयोग कनेक्शन का परीक्षण करने और उसे लेबल देने के लिए करते हैं।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`GetMeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMeResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getMe उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new पर रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 

DefaultAPI.getMe(tenantId: tenantId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]