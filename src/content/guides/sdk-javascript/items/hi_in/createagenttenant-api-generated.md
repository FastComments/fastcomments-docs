बिना मानवीय साइनअप के AI एजेंट के लिए एक नया ट्रायल अकाउंट बनाता है। इसे कॉल करने के लिए किसी API key की आवश्यकता नहीं है।

रिस्पॉन्स में tenant id, एक ऐसी API key जो REST API और MCP server के विरुद्ध तुरंत काम करती है, और एक claim URL शामिल होता है। claim URL उस व्यक्ति को दें जिसके लिए आप काम कर रहे हैं: FastComments में लॉग इन होने के दौरान इसे खोलने पर अकाउंट उससे जुड़ जाता है। बिना claim किए गए अकाउंट, और उनकी keys, बनाए जाने के 72 घंटे बाद डिलीट कर दिए जाते हैं। Claim होने तक, अकाउंट पर सामान्य ट्रायल सीमाएँ लागू रहती हैं।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| createAgentTenantBody | CreateAgentTenantBody | हाँ |  |

## रिस्पॉन्स

रिटर्न: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateAgentTenantResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'createAgentTenant उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantBody: CreateAgentTenantBody = {
  name: "Acme Corp",
  domain: "acme.example.com",
  contactEmail: "admin@acme.example.com", // वैकल्पिक
  planId: 3 // वैकल्पिक
};

const result: CreateAgentTenantResponse = await createAgentTenant(tenantBody);
[inline-code-end]