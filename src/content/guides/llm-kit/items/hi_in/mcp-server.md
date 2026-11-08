FastComments एक होस्टेड Model Context Protocol (MCP) सर्वर चलाता है ताकि AI असिस्टेंट और एजेंटिक क्लाइंट सीधे FastComments API को कॉल कर सकें। MCP सर्वर द्वारा उजागर किया गया हर टूल सार्वजनिक OpenAPI स्पेक से ऑटो-जेनरेट होता है, इसलिए जो कुछ भी REST API कर सकता है, वह एक MCP क्लाइंट भी कर सकता है।

एंडपॉइंट स्टेटलेस और streamable-HTTP आधारित है। जीवित रखने के लिए कोई सेशन नहीं है और प्रति क्लाइंट कोई सर्वर-साइड स्टेट नहीं है।

### एंडपॉइंट

[inline-code-attrs-start title = 'MCP एंडपॉइंट'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### OAuth के साथ कनेक्ट करें

कोई भी MCP क्लाइंट जो OAuth के साथ रिमोट सर्वर का समर्थन करता है (Claude, ChatGPT, Claude Code, Cursor, और अन्य) FastComments की ओर से किसी सेटअप के बिना ऊपर दिए गए एंडपॉइंट से कनेक्ट हो सकता है। क्लाइंट Dynamic Client Registration के माध्यम से स्वयं को पंजीकृत करता है या Client ID Metadata Document के साथ स्वयं की पहचान करता है, एक ब्राउज़र खोलता है ताकि आप FastComments में साइन इन कर सकें और एक्सेस को स्वीकृति दे सकें, और उस खाते से बंधे टोकन को प्राप्त करता है जिसमें आप साइन इन थे।

डिस्कवरी दस्तावेज़ मानक स्थानों पर हैं:

[inline-code-attrs-start title = 'डिस्कवरी'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

कनेक्शन को स्वीकृति देने के लिए आपके उपयोगकर्ता को खाते पर API Admin अनुमति की आवश्यकता है। यदि आप कई खातों का प्रबंधन करते हैं, तो स्वीकृति देने से पहले डैशबोर्ड में सही खाते पर स्विच करें।

एक क्लाइंट `read` स्कोप, `write` स्कोप, या दोनों का अनुरोध कर सकता है। जो क्लाइंट कुछ भी अनुरोध नहीं करता उसे दोनों मिलते हैं। डेटा बदलने वाले टूल केवल-पढ़ने वाले टोकन को उपलब्ध नहीं कराए जाते।

डैशबोर्ड में तैयार-पेस्ट स्निपेट के साथ एक सेटअप हेल्पर है। **Integrate -> MCP Server** खोलें, या सीधे इसे देखें:

[inline-code-attrs-start title = 'सेटअप पेज'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

एक कमांड के साथ FastComments सर्वर को पंजीकृत करें, फिर साइन इन करने और उपलब्ध टूल्स की सूची देखने के लिए सेशन के अंदर `/mcp` चलाएँ:

[inline-code-attrs-start title = 'Claude Code सेटअप'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor और अन्य कॉन्फ़िग-फ़ाइल क्लाइंट

इस ब्लॉक को अपने क्लाइंट के MCP सर्वर कॉन्फ़िग में जोड़ें (Cursor के लिए `mcp.json`)। पहली बार उपयोग पर साइन इन करने के लिए क्लाइंट एक ब्राउज़र खोलता है।

[inline-code-attrs-start title = 'MCP क्लाइंट कॉन्फ़िग'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "mcpServers": {
    "fastcomments": {
      "type": "http",
      "url": "https://fastcomments.com/mcp"
    }
  }
}
[inline-code-end]

### एक्सेस रद्द करना

हर स्वीकृत कनेक्शन डैशबोर्ड में **Integrate -> Connected Apps** के अंतर्गत सूचीबद्ध है। एक को रद्द करने से उस एप्लिकेशन के पास मौजूद हर टोकन अमान्य हो जाता है। एप्लिकेशन कनेक्ट होते समय स्वयं को पंजीकृत करते हैं और FastComments उनकी समीक्षा नहीं करता है, इसलिए जो कुछ भी आप पहचानते नहीं हैं उसे रद्द करें।

### REST API के साथ टोकन का उपयोग

MCP क्लाइंट द्वारा प्राप्त एक्सेस टोकन एक सामान्य FastComments API क्रेडेंशियल है। यह हर `/api/v1` एंडपॉइंट पर बियरर टोकन के रूप में काम करता है, इसलिए MCP के माध्यम से कनेक्ट हुआ एप्लिकेशन REST API को भी सीधे कॉल कर सकता है:

[inline-code-attrs-start title = 'बियरर टोकन'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

टेनेंट टोकन द्वारा निहित है। एक `tenantId` अभी भी पास किया जा सकता है लेकिन यह मेल खाना चाहिए। `GET` अनुरोधों को `read` स्कोप की आवश्यकता होती है और बाकी सब कुछ को `write` स्कोप की।

### API कुंजी के साथ कनेक्ट करें

वे क्लाइंट जो ब्राउज़र साइन-इन पूरा नहीं कर सकते, जैसे हेडलेस सर्वर, इसके बजाय API कुंजी से प्रमाणित कर सकते हैं। `tenantId` और `API_KEY` को क्वेरी पैरामीटर के रूप में पास करें, या `x-tenant-id` और `x-api-key` HTTP हेडर के रूप में यदि आपका क्लाइंट कस्टम हेडर का समर्थन करता है:

[inline-code-attrs-start title = 'API कुंजी एंडपॉइंट'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

सेटअप पेज आपकी प्रत्येक API कुंजी के लिए यह URL जेनरेट करता है।

### सुरक्षा

एक एंडपॉइंट URL जिसमें API कुंजी होती है एक रहस्य है: इसे सार्वजनिक चैट, स्क्रीनशॉट, या कमिट में पेस्ट न करें। यदि कोई कुंजी उजागर हो जाती है, तो अपने डैशबोर्ड में API Keys पेज पर इसे रोटेट करें। OAuth टोकन में ऐसा कोई जोखिम नहीं होता क्योंकि वे एक एप्लिकेशन से बंधे होते हैं और Connected Apps से रद्द किए जा सकते हैं।