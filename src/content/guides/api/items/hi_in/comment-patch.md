[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

यह API एंडपॉइंट एक सिंगल कमेंट को अपडेट करने की क्षमता प्रदान करता है।

नोट्स:

- यह API चाहें तो कमेंट विजेट को "live" अपडेट कर सकता है (इससे बेस `creditsCost` `1` से बढ़कर `2` हो जाता है)।
  - इससे पेजों के बीच कमेंट्स को "live" माइग्रेट किया जा सकता है (`urlId` बदलकर)।
  - माइग्रेशन पर अतिरिक्त `2` क्रेडिट खर्च होते हैं क्योंकि पेज प्री-कैलकुलेट किए जाते हैं और यह CPU-इंटेंसिव है।
- क्रिएट API के विपरीत, यह API ईमेल दिए जाने पर भी हमारे सिस्टम में ऑटोमैटिकली यूज़र ऑब्जेक्ट नहीं बनाएगा।
- इस API के माध्यम से अपडेट किए गए कमेंट्स की चाहें तो स्पैम जांच भी की जा सकती है।
- अधिकतम कमेंट लंबाई जैसी कॉन्फ़िगरेशन, यदि Customization Rule एडमिन पेज के माध्यम से कॉन्फ़िगर की गई है, तो यहाँ भी लागू होगी।
- यूज़र्स को उनके कमेंट टेक्स्ट को अपडेट करने की अनुमति देने के लिए, आप बस रिक्वेस्ट बॉडी में `comment` स्पेसिफाई कर सकते हैं। हम परिणामी `commentHTML` जनरेट करेंगे।
  - यदि आप `comment` और `commentHTML` दोनों डिफाइन करते हैं तो हम HTML ऑटोमैटिकली जनरेट नहीं करेंगे।
  - यदि यूज़र अपने नए टेक्स्ट में मेंशन्स या हैशटैग जोड़ता है, तो उन्हें अभी भी `POST` API की तरह प्रोसेस किया जाएगा।
- किसी कमेंट पर `commenterEmail` अपडेट करते समय, `userId` भी स्पेसिफाई करना सबसे अच्छा है। अन्यथा, आपको यह सुनिश्चित करना होगा कि इस ईमेल वाला यूज़र आपके टेनेंट से संबंधित है, नहीं तो रिक्वेस्ट विफल हो जाएगी।  
- यदि टारगेट कमेंट लॉक है (`isLocked: true`), तो रिक्वेस्ट `code: 'locked'` के साथ रिजेक्ट कर दी जाती है। पहले कमेंट को अनलॉक करें, उसे अपडेट करें, फिर चाहें तो दोबारा लॉक करें।


[inline-code-attrs-start title = 'न्यूनतम कमेंट PATCH cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'कमेंट PATCH रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** अपडेट करने वाला यूज़र। चाहें तो इसका उपयोग यह जांचने के लिए किया जा सकता है कि वे कमेंट एडिट कर सकते हैं।  **/
    contextUserId?: string
	/** क्या हमें जांचना चाहिए कि नया कमेंट स्पैम जैसा दिखता है?  **/
    doSpamCheck?: 'true' | 'false'
	/** क्या कमेंट उसी urlId वाले कमेंट विजेट के इंस्टेंस देख रहे यूज़र्स को "live" दिखना चाहिए। NOTE: क्रेडिट लागत 1 से बढ़कर 2 हो जाती है। **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'कमेंट PATCH रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found' | 'locked'
    /** विफलता पर शामिल। **/
    reason?: string
}
[inline-code-end]