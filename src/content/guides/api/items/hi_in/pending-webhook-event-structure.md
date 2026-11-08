एक `PendingWebhookEvent` ऑब्जेक्ट एक कतार में लंबित (pending) वेबहुक इवेंट का प्रतिनिधित्व करता है।

`PendingWebhookEvent` ऑब्जेक्ट स्वचालित रूप से बनाए जाते हैं और इन्हें API के माध्यम से मैन्युअल रूप से नहीं बनाया जा सकता। ये एक वर्ष के बाद समाप्त (expire) भी हो जाते हैं।
इन्हें डिलीट किया जा सकता है, जिससे यह कार्य कतार से हट जाता है।

अलग-अलग इवेंट प्रकार होते हैं - `eventType` (`OutboundSyncEventType`) और `type` (`OutboundSyncType`) देखें।

इस API का एक सामान्य उपयोग कस्टम मॉनिटरिंग लागू करना है। आप शायद `/count` एंडपॉइंट को समय-समय पर कॉल करके दिए गए फ़िल्टरों के लिए लंबित संख्या पोल करना चाहें।

`PendingWebhookEvent` ऑब्जेक्ट की संरचना इस प्रकार है:

[inline-code-attrs-start title = 'PendingWebhookEvent संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum OutboundSyncEventType {
    Create: 0,
    Delete: 1,
    Update: 2
}

enum OutboundSyncType {
    /** WordPress-विशिष्ट सिंक कार्य। **/
    WP: 0,
    Webhook: 1
}

interface PendingWebhookEvent {
    id: string
    /** इवेंट से जुड़ी कमेंट की id। **/
    commentId: string
    /** इवेंट के समय इवेंट के लिए कमेंट ऑब्जेक्ट। हमने इन्हें नवंबर 2023 से जोड़ना शुरू किया। **/
    comment: Comment
    /** एक बाहरी id जो कमेंट से जुड़ी हो सकती है। **/
    externalId: string | null
    createdAt: Date
    tenantId: string
    attemptCount: number
    /** पहले प्रयास से पहले, और हर विफलता के बाद सेट होता है। **/
    nextAttemptAt: Date
    /** क्या यह निर्माण, डिलीशन, या अपडेट इवेंट है... **/
    eventType: OutboundSyncEventType
    /** करने के लिए सिंक का प्रकार (WordPress, API कॉल, आदि)। **/
    type: OutboundSyncType
    /** वह डोमेन जो कमेंट से मेल खाया। हम इस डोमेन का उपयोग API कुंजी चुनने के लिए करते हैं। **/
    domain: string
    /** हुई अंतिम त्रुटि। यह प्रकार अनटाइप्ड है और जो कुछ हुआ उसका एक "डंप" है। आमतौर पर इसमें statusCode, body, और एक headers मैप के साथ एक ऑब्जेक्ट होता है। **/
    lastError: object | null
}
[inline-code-end]