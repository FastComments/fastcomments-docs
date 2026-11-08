## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| userId | string | query | नहीं |  |
| state | number | query | नहीं |  |
| skip | number | query | नहीं |  |
| limit | number | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTicketsResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getTickets उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new पर रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (वैकल्पिक)
let state = 987 // Double |  (वैकल्पिक)
let skip = 987 // Double |  (वैकल्पिक)
let limit = 987 // Double |  (वैकल्पिक)

DefaultAPI.getTickets(tenantId: tenantId, options: DefaultAPI.GetTicketsOptions(userId: userId, state: state, skip: skip, limit: limit)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]