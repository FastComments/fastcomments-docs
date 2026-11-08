एक `AuditLog` एक ऐसा ऑब्जेक्ट है जो उन टेनेंट्स के लिए ऑडिट की गई घटना को दर्शाता है जिनके पास इस सुविधा तक पहुँच है।

AuditLog ऑब्जेक्ट की संरचना इस प्रकार है:

[inline-code-attrs-start title = 'AuditLog संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLog {
    id: string;
    /** घटना किसने की। **/
    userId?: string;
    username?: string;
    resourceName: string;
    crudType: 'c' | 'r' | 'u' | 'd' | 'login';
    from: string;
    url?: string;
    ip?: string;
    /** वह ब्राउज़र जिसने घटना की, जब वह किसी ब्राउज़र से आई हो। **/
    ua?: string;
    /** उस सेशन का हैश जिससे घटना आई, एक व्यक्ति की क्रियाओं को सहसंबंधित करने के लिए। कभी भी सेशन स्वयं नहीं। **/
    sIdHashed?: string;
    when: string;
    description?: string;
    serverStartDate: string;
    /** उस ऑब्जेक्ट की id जिस पर घटना की गई, इसके विपरीत कि किसने की। **/
    targetId?: string;
    /** उस ऑब्जेक्ट के लिए मानव-पठनीय लेबल, जैसे "jsmith (jsmith@example.com)"। **/
    targetLabel?: string;
    objectDetails?: object;
}
[inline-code-end]

`targetId` और `targetLabel` यह बताते हैं कि घटना किस पर की गई; `userId` और `username` यह बताते हैं कि उसे किसने की। अपडेट के लिए, `objectDetails.changes` में वास्तव में क्या बदला, इसका एक `{field: {from, to}}` मैप होता है।

ऑडिट लॉग अपरिवर्तनीय (immutable) है। इसे मैन्युअल रूप से लिखा भी नहीं जा सकता। FastComments.com ही तय कर सकता है कि ऑडिट लॉग में कब लिखा जाए। हालाँकि, आप इस API के माध्यम से इसे पढ़ सकते हैं।

ऑडिट लॉग में घटनाएँ दो साल बाद समाप्त हो जाती हैं।