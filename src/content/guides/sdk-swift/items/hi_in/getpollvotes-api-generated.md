एक पोल की गिनती के पीछे के व्यक्तिगत वोट, सबसे पुराने पहले।

एक पोल एक कमेंट से संबंधित होता है, इसलिए वोट हमेशा एक बार में एक पोल के लिए पढ़े जाते हैं - commentId आवश्यक है। इससे हर क्वेरी उन इंडेक्स पर ही रहती है जो कलेक्शन में पहले से मौजूद हैं।

पोल की प्राइवेसी का पालन करता है: एक अनाम पोल के वोट नहीं पढ़े जा सकते (poll-anonymous), यहाँ या id द्वारा।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| commentId | string | query | हाँ |  |
| voterId | string | query | नहीं |  |
| optionId | string | query | नहीं |  |
| skip | integer | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVotesResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getPollVotes उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा में हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new पर रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let voterId = "voterId_example" // String |  (optional)
let optionId = "optionId_example" // String |  (optional)
let skip = 987 // Int |  (optional)

DefaultAPI.getPollVotes(tenantId: tenantId, commentId: commentId, options: DefaultAPI.GetPollVotesOptions(voterId: voterId, optionId: optionId, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]