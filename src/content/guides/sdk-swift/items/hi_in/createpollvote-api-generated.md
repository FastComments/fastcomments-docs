किसी पोल पर वोट दर्ज करें, या मौजूदा वोट को किसी अन्य विकल्प पर स्थानांतरित करें। एक वोटर के पास प्रति पोल अधिकतम एक वोट होता है, इसलिए उसी वोटर के लिए इसे दोबारा कॉल करने पर नया वोट जोड़ने के बजाय उनका वोट स्थानांतरित हो जाता है।

यह साइट की पोल सेटिंग्स का पालन करता है: यदि वोटिंग केवल लॉग-इन उपयोगकर्ताओं के लिए सेट है, तो केवल anonUserId वाला वोट अस्वीकार कर दिया जाता है, और गुमनाम वोट प्रति IP प्रति पोल दर सीमित (rate limited) होते हैं।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |

## प्रतिक्रिया

रिटर्न: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreatePollVoteResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'createPollVote उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createPollVoteBody = CreatePollVoteBody(commentId: "commentId_example", optionId: "optionId_example", userId: "userId_example", anonUserId: "anonUserId_example", ip: "ip_example") // CreatePollVoteBody | 

DefaultAPI.createPollVote(tenantId: tenantId, createPollVoteBody: createPollVoteBody) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]