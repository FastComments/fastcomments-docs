एक `NotificationCount` ऑब्जेक्ट किसी उपयोगकर्ता के लिए अपठित नोटिफिकेशन की संख्या और मेटाडेटा को दर्शाता है।

यदि कोई अपठित नोटिफिकेशन नहीं है, तो उपयोगकर्ता के लिए कोई `NotificationCount` नहीं होगा।

`NotificationCount` ऑब्जेक्ट स्वचालित रूप से बनाए जाते हैं और API के माध्यम से नहीं बनाए जा सकते। ये एक वर्ष बाद समाप्त (expire) हो जाते हैं।

आप उपयोगकर्ता का `NotificationCount` डिलीट करके उसकी अपठित नोटिफिकेशन संख्या साफ़ कर सकते हैं।

`NotificationCount` ऑब्जेक्ट की संरचना इस प्रकार है:

[inline-code-attrs-start title = 'NotificationCount संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCount {
    id: string // user id
    count: number
    createdAt: string // date string
    expireAt: string // date string
}
[inline-code-end]