बिना मानवीय साइनअप के AI एजेंट के लिए एक नया ट्रायल अकाउंट बनाता है। इसे कॉल करने के लिए किसी API key की आवश्यकता नहीं है।

रिस्पॉन्स में tenant id, एक API key शामिल है जो REST API और MCP server के विरुद्ध तुरंत काम करती है, और एक claim URL। Claim URL उस व्यक्ति को दें जिसके लिए आप काम कर रहे हैं: FastComments में लॉग इन होने के दौरान इसे खोलने पर अकाउंट उससे जुड़ जाता है। बिना claimed अकाउंट, और उनकी keys, बनाए जाने के 72 घंटे बाद डिलीट कर दिए जाते हैं। Claim होने तक, अकाउंट पर मानक ट्रायल सीमाएँ लागू रहती हैं।

## Response

Returns: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateAgentTenantResponse.swift)

## Example

[inline-code-attrs-start title = 'createAgentTenant उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let createAgentTenantBody = CreateAgentTenantBody(name: "name_example", domains: ["domains_example"], agent: "agent_example") // CreateAgentTenantBody | 

DefaultAPI.createAgentTenant(createAgentTenantBody: createAgentTenantBody) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]