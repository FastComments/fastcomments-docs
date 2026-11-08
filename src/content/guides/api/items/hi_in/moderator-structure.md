एक `Moderator` ऑब्जेक्ट एक मॉडरेटर के लिए कॉन्फ़िगरेशन का प्रतिनिधित्व करता है।

मॉडरेटर के तीन प्रकार होते हैं:

1. वे एडमिनिस्ट्रेटर उपयोगकर्ता जिनके पास `isCommentModeratorAdmin` फ्लैग होता है।
2. `isCommentModeratorAdmin` फ्लैग वाले SSO उपयोगकर्ता।
3. सामान्य टिप्पणीकार, या FastComments.com उपयोगकर्ता, जिन्हें मॉडरेटर के रूप में आमंत्रित किया गया हो।

`Moderator` संरचना का उपयोग उपयोग केस `3` की मॉडरेशन स्थिति (Moderation State) को दर्शाने के लिए किया जाता है।

यदि आप API के माध्यम से किसी उपयोगकर्ता को मॉडरेटर बनने के लिए आमंत्रित करना चाहते हैं, तो एक `Moderator` बनाकर और उन्हें `inviting` करके `Moderator` API का उपयोग करें।

यदि उपयोगकर्ता के पास FastComments.com खाता नहीं है, तो आमंत्रण ईमेल उन्हें सेटअप करने में मदद करेगा। यदि उनके पास पहले से खाता है, तो उन्हें आपके टेनेंट (tenant) तक मॉडरेशन एक्सेस दिया जाएगा और `Moderator` ऑब्जेक्ट का `userId` उनके उपयोगकर्ता की ओर इशारा करने के लिए अपडेट कर दिया जाएगा। आपको उनके उपयोगकर्ता तक API एक्सेस नहीं मिलेगा, क्योंकि इस स्थिति में वह उपयोगकर्ता स्वयं का होता है और FastComments.com द्वारा प्रबंधित किया जाता है।

यदि आप उपयोगकर्ता के खाते का पूर्ण प्रबंधन चाहते हैं, तो हम या तो SSO का उपयोग करने, या उन्हें [Tenant User](https://fastcomments.com/auth/my-account/users) के रूप में जोड़ने और फिर उनके आंकड़े ट्रैक करने के लिए एक `Moderator` ऑब्जेक्ट जोड़ने की सलाह देते हैं।

`Moderator` संरचना का उपयोग उपयोग केस `1` और `2` के लिए आंकड़े ट्रैकिंग तंत्र के रूप में किया जा सकता है। उपयोगकर्ता बनाने के बाद, उनका `userId` निर्दिष्ट करते हुए एक `Moderator` ऑब्जेक्ट जोड़ें और उनके आंकड़े [Comment Moderators Page](https://fastcomments.com/auth/my-account/moderate-comments/moderators) पर ट्रैक किए जाएंगे।

`Moderator` ऑब्जेक्ट की संरचना इस प्रकार है:

[inline-code-attrs-start title = 'Moderator संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Moderator {
    name: string
    email: string
    tenantId: string
    userId?: string|null
    acceptedInvite?: boolean
    markReviewedCount?: number
    deletedCount?: number
    markedSpamCount?: number
    approvedCount?: number
    editedCount?: number
    bannedCount?: number
    createdAt: string
    moderationGroupIds?: string[]|null
}
[inline-code-end]