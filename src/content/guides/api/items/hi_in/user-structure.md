`User` एक ऑब्जेक्ट है जो सभी उपयोगकर्ताओं के सबसे सामान्य गुणों को दर्शाता है।

ध्यान रखें कि FastComments पर उपयोगकर्ताओं के लिए हमारे पास कई अलग-अलग उपयोग के मामले हैं:

- Secure SSO
- Simple SSO
- Tenant Users (उदाहरण के लिए: Administrators)
- Commenters

यह API **Commenters** और **Simple SSO** के माध्यम से बनाए गए उपयोगकर्ताओं के लिए है। मूल रूप से, आपकी साइट के माध्यम से बनाया गया कोई भी उपयोगकर्ता इस API के माध्यम से एक्सेस किया जा सकता है। Tenant Users को भी इस तरह प्राप्त किया जा सकता है, लेकिन `/tenant-users/` API के साथ इंटरैक्ट करने पर आपको अधिक जानकारी मिलेगी।

`Secure SSO` के लिए कृपया `/sso-users/` API का उपयोग करें।

आप इन प्रकार के उपयोगकर्ताओं को अपडेट नहीं कर सकते। उन्होंने अपना खाता आपकी साइट के माध्यम से बनाया है, इसलिए हम कुछ बुनियादी केवल-पढ़ने योग्य (read-only) एक्सेस प्रदान करते हैं, लेकिन
आप बदलाव नहीं कर सकते। यदि आप इस प्रकार का फ्लो चाहते हैं - तो आपको `Secure SSO` सेटअप करना होगा।

`User` ऑब्जेक्ट की संरचना इस प्रकार है:

[inline-code-attrs-start title = 'User संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** यह वही id है जो comment ऑब्जेक्ट्स पर userId के रूप में उपयोग की जाती है। **/
    id: string
    username: string
    /** उदाहरण के लिए, कमेंटर के ब्लॉग का लिंक। **/
    websiteUrl?: string
    email: string
    signUpDate: number
    createdFromUrlId: string
    createdFromTenantId: string
    avatarSrc?: string
    locale: FastCommentsLocale
    displayLabel?: string
    karma?: number
}
[inline-code-end]