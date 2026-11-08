एक `Subscription` ऑब्जेक्ट किसी उपयोगकर्ता की सदस्यता (subscription) को दर्शाता है।

`Subscription` ऑब्जेक्ट तब बनाए जाते हैं जब कोई उपयोगकर्ता कमेंट विजेट में नोटिफिकेशन बेल पर क्लिक करता है और "Subscribe to this page" पर क्लिक करता है।

सदस्यताएँ API के माध्यम से भी बनाई जा सकती हैं।

एक `Subscription` ऑब्जेक्ट होने से, संबंधित पेज के रूट पर नए कमेंट छोड़े जाने पर, उस `Subscription` के लिए `Notification` ऑब्जेक्ट बनते हैं और ईमेल भेजे जाते हैं। ईमेल भेजना उपयोगकर्ता के प्रकार पर निर्भर करता है। सामान्य उपयोगकर्ताओं के लिए यह `optedInNotifications` पर निर्भर करता है। SSO उपयोगकर्ताओं के लिए यह `optedInSubscriptionNotifications` पर निर्भर करता है। ध्यान दें कि कुछ एप्लिकेशन में वेब-एक्सेसिबल पेज की अवधारणा नहीं हो सकती, ऐसे में बस `urlId` को उस आइटम की id पर सेट करें जिसकी आप सदस्यता ले रहे हैं (कमेंट विजेट को जो `urlId` पास करेंगे, वही मान)।

`Subscription` ऑब्जेक्ट की संरचना इस प्रकार है:

[inline-code-attrs-start title = 'Subscription संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** SSO के साथ, user id का फॉर्मैट `<tenant id>:<user id>` होता है। **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // date string
}
[inline-code-end]