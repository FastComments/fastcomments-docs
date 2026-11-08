## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ | वह टेनेंट जिसके लिए ऑडिट लॉग पढ़ने हैं। |
| limit | integer | query | नहीं | कितने लॉग लौटाने हैं। अधिकतम 10000, डिफ़ॉल्ट 1000। |
| skip | integer | query | नहीं | पेजिंग के लिए कितने लॉग छोड़ने हैं। |
| order | string | query | नहीं | इवेंट समय पर सॉर्ट दिशा। डिफ़ॉल्ट DESC (नवीनतम पहले)। |
| after | integer | query | नहीं |  |
| before | integer | query | नहीं |  |
| username | string | query | नहीं | केवल इस यूज़रनेम द्वारा किए गए इवेंट लौटाएँ। |
| ip | string | query | नहीं | केवल इस IP पते से इवेंट लौटाएँ। |
| crudType | string | query | नहीं | केवल इस प्रकार के इवेंट लौटाएँ: c, r, u, d, या login। |
| resourceName | string | query | नहीं | केवल इस रिसोर्स के लिए इवेंट लौटाएँ, जैसे User या Moderator। |
| targetId | string | query | नहीं | केवल वे इवेंट लौटाएँ जिनकी प्रभावित वस्तु की यह id है। |
| target | string | query | नहीं | प्रभावित वस्तु के लेबल पर केस-असंवेदनशील सबस्ट्रिंग मैच, जैसे यूज़रनेम या ईमेल। |
| includeManagedTenants | boolean | query | नहीं | उन टेनेंट के इवेंट भी लौटाएँ जिन्हें यह टेनेंट प्रबंधित करता है। |

## प्रतिक्रिया

लौटाता है: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogsResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getAuditLogs उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new पर रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | वह टेनेंट जिसके लिए ऑडिट लॉग पढ़ने हैं।
let limit = 987 // Int | कितने लॉग लौटाने हैं। अधिकतम 10000, डिफ़ॉल्ट 1000। (optional)
let skip = 987 // Int | पेजिंग के लिए कितने लॉग छोड़ने हैं। (optional)
let order = SORT_DIR() // SORTDIR | इवेंट समय पर सॉर्ट दिशा। डिफ़ॉल्ट DESC (नवीनतम पहले)। (optional)
let after = 987 // Int64 |  (optional)
let before = 987 // Int64 |  (optional)
let username = "username_example" // String | केवल इस यूज़रनेम द्वारा किए गए इवेंट लौटाएँ। (optional)
let ip = "ip_example" // String | केवल इस IP पते से इवेंट लौटाएँ। (optional)
let crudType = "crudType_example" // String | केवल इस प्रकार के इवेंट लौटाएँ: c, r, u, d, या login। (optional)
let resourceName = "resourceName_example" // String | केवल इस रिसोर्स के लिए इवेंट लौटाएँ, जैसे User या Moderator। (optional)
let targetId = "targetId_example" // String | केवल वे इवेंट लौटाएँ जिनकी प्रभावित वस्तु की यह id है। (optional)
let target = "target_example" // String | प्रभावित वस्तु के लेबल पर केस-असंवेदनशील सबस्ट्रिंग मैच, जैसे यूज़रनेम या ईमेल। (optional)
let includeManagedTenants = true // Bool | उन टेनेंट के इवेंट भी लौटाएँ जिन्हें यह टेनेंट प्रबंधित करता है। (optional)

DefaultAPI.getAuditLogs(tenantId: tenantId, options: DefaultAPI.GetAuditLogsOptions(limit: limit, skip: skip, order: order, after: after, before: before, username: username, ip: ip, crudType: crudType, resourceName: resourceName, targetId: targetId, target: target, includeManagedTenants: includeManagedTenants)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]