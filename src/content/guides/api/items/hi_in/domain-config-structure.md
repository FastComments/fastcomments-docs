एक `DomainConfig` ऑब्जेक्ट किसी टेनेंट के लिए एक डोमेन की कॉन्फ़िगरेशन दर्शाता है।

`DomainConfig` ऑब्जेक्ट की संरचना इस प्रकार है:

[inline-code-attrs-start title = 'डोमेन कॉन्फ़िग संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** एक डोमेन, URL नहीं, जैसे "fastcomments.com" या "www.example.com"। यदि सबडोमेन तक सीमित करना चाहा जाए तो सबडोमेन शामिल किया जा सकता है। अधिकतम 1000 वर्ण। **/
    domain: string
    /** ईमेल भेजते समय उपयोग किया जाने वाला From-Name। **/
    emailFromName?: string
    /** ईमेल भेजते समय उपयोग किया जाने वाला From-Email। सुनिश्चित करें कि SPF इस एट्रिब्यूट में उपयोग किए गए डोमेन के रूप में ईमेल भेजने के लिए mail.fastcomments.com को अनुमति देने के लिए सेटअप है। **/
    emailFromEmail?: string
    /** READONLY। जब ऑब्जेक्ट बनाया गया था। **/
    createdAt: string
    /** इस डोमेन से संबंधित लोगो। ईमेल में उपयोग होता है। HTTPS का उपयोग करें। **/
    logoSrc?: string
    /** इस डोमेन से संबंधित एक छोटा लोगो। HTTPS का उपयोग करें। **/
    logoSrc100px?: string
    /** SSO ONLY। भेजे गए हर ईमेल के फुटर में उपयोग किया जाने वाला URL। "[userId]" वेरिएबल का समर्थन करता है। **/
    footerUnsubscribeURL?: string
    /** SSO ONLY। भेजे गए हर ईमेल में उपयोग किए जाने वाले हेडर। उदाहरण के लिए, डिलीवरी में सुधार के लिए अनसब्सक्राइब से संबंधित हेडर सेट करने के लिए उपयोगी। इस Record में List-Unsubscribe एंट्री, यदि मौजूद हो, "[userId]" वेरिएबल का समर्थन करती है। **/
    emailHeaders?: Record<string, string>
    /** सभी अनसब्सक्राइब लिंक अक्षम करें। अनुशंसित नहीं, डिलीवरी दरों को नुकसान हो सकता है। **/
    disableUnsubscribeLinks?: boolean
    /** DKIM कॉन्फ़िगरेशन। **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'DKIM कॉन्फ़िग संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** आपके DKIM रिकॉर्ड में डोमेन नाम। **/
    domainName: string
    /** उपयोग करने के लिए DKIM की सिलेक्टर। **/
    keySelector: string
    /** सार्वजनिक कुंजी, PEM फॉर्मेट में। GET प्रतिक्रियाओं में लौटाया जाता है। **/
    publicKey: string
    /** @deprecated अब API प्रतिक्रियाओं में लौटाया नहीं जाता। पिछली संगतता के लिए लेखन पर स्वीकार किया जाता है। **/
    privateKey?: string
}
[inline-code-end]

### प्रमाणीकरण के लिए

डोमेन कॉन्फ़िगरेशन का उपयोग यह निर्धारित करने के लिए किया जाता है कि कौन सी साइटें आपके खाते के लिए FastComments विजेट होस्ट कर सकती हैं। यह प्रमाणीकरण का एक बुनियादी रूप है, जिसका अर्थ है कि किसी भी डोमेन कॉन्फ़िगरेशन को जोड़ने या हटाने से प्रोडक्शन में आपकी FastComments इंस्टॉलेशन की उपलब्धता प्रभावित हो सकती है।

किसी ऐसे डोमेन के लिए `Domain Config` की `domain` प्रॉपर्टी को हटाएं या अपडेट न करें जो वर्तमान में उपयोग में है, जब तक कि उस डोमेन को अक्षम करना इरादा न हो।

इसका व्यवहार [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains) से डोमेन हटाने जैसा ही है।

यह भी ध्यान दें कि `My Domains` UI से डोमेन हटाने से उस डोमेन के लिए इस UI के माध्यम से जोड़ी गई कोई भी संबंधित कॉन्फ़िगरेशन हट जाएगी।

### ईमेल अनुकूलन के लिए

ईमेल फुटर में अनसब्सक्राइब लिंक, और कई ईमेल क्लाइंट द्वारा प्रदान की जाने वाली वन-क्लिक-अनसब्सक्राइब सुविधा, को इस API के माध्यम से क्रमशः `footerUnsubscribeURL` और `emailHeaders` को परिभाषित करके कॉन्फ़िगर किया जा सकता है।

### DKIM के लिए

अपने DKIM DNS रिकॉर्ड परिभाषित करने के बाद, परिभाषित संरचना का उपयोग करके अपने DKIM कॉन्फ़िगरेशन के साथ DomainConfig को बस अपडेट करें।