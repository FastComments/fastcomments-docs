एक `EmailTemplate` ऑब्जेक्ट किसी tenant के लिए कस्टम ईमेल टेम्पलेट की कॉन्फ़िगरेशन को दर्शाता है।

सिस्टम उपयोग करने के लिए ईमेल टेम्पलेट का चयन इस प्रकार करेगा:

- इसके type identifier द्वारा, जिसे हम `emailTemplateId` कहते हैं। ये constants होते हैं।
- `domain` द्वारा। हम पहले संबंधित ऑब्जेक्ट (जैसे `Comment`) से जुड़े domain के लिए टेम्पलेट खोजने का प्रयास करेंगे, और यदि कोई मैच नहीं मिलता है तो हम ऐसा टेम्पलेट खोजने का प्रयास करेंगे जिसमें domain null या `*` हो।

`EmailTemplate` ऑब्जेक्ट की संरचना इस प्रकार है:

[inline-code-attrs-start title = 'ईमेल टेम्पलेट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplate {
    id: string
    tenantId: string
    emailTemplateId: string
    displayName: string
    /** READONLY **/
    createdAt: string
    /** READONLY **/
    updatedAt: string
    /** READONLY **/
    updatedByUserId: string
    /** वह domain जिससे टेम्पलेट संबद्ध होना चाहिए। **/
    domain?: string | '*' | null
    /** EJS सिंटैक्स में ईमेल टेम्पलेट की सामग्री। **/
    ejs: string
    /** प्रत्येक समर्थित locale के लिए, ओवरराइड की गई अनुवाद keys से values तक का मैप। **/
    translationOverridesByLocale: Record<string, Record<string, string>>
    /** एक ऑब्जेक्ट जो टेम्पलेट के render context को दर्शाता है। **/
    testData: object
}
[inline-code-end]

### नोट्स

- आप मान्य `emailTemplateId` values `/definitions` endpoint से प्राप्त कर सकते हैं।
- `/definitions` endpoint में डिफ़ॉल्ट अनुवाद और test data भी शामिल हैं।
- अमान्य संरचना या test data के साथ टेम्पलेट सहेजने में विफल हो जाएंगे।