Page Reacts आपके उपयोगकर्ताओं को किसी पेज को लाइक करने, या आपके अपने रिएक्शन इमेज के सेट के साथ उस पर प्रतिक्रिया देने की सुविधा देते हैं। [Page Reacts widget](/guide-page-reacts.html) और Floating Likes widget इन endpoints पर बने हैं, और आप इन्हें स्वयं कॉल करके अपना लाइक बटन बना सकते हैं।

इस गाइड के बाकी हिस्सों के विपरीत, Page Reacts endpoints सार्वजनिक (public) हैं। इन्हें आपके उपयोगकर्ताओं के ब्राउज़र से कॉल किया जाता है, इनमें कोई API key नहीं लगती, और ये कोई API credits खर्च नहीं करते। हर react अनुरोध करने वाले उपयोगकर्ता से जुड़ा होता है, इसलिए कोई उपयोगकर्ता केवल अपना ही react जोड़ या हटा सकता है।

Endpoints के दो सेट हैं:

- `/page-reacts/v1/likes/:tenantId` - प्रति उपयोगकर्ता प्रति पेज एक "like"। इन्हें लाइक बटन के लिए उपयोग करें।
- `/page-reacts/v2/:tenantId` - प्रति पेज एकाधिक reactions, प्रत्येक आपके चुने हुए एक छोटे `id` से पहचाना जाता है (उदाहरण के लिए `heart` या `laugh`)।

दोनों हमारे SDKs में `PublicApi` के हिस्से के रूप में भी उपलब्ध हैं, उदाहरण के लिए [JavaScript SDK](/guide-sdk-javascript.html) में `getV1PageLikes`, `createV1PageReact`, और `deleteV1PageReact`।

### किसने प्रतिक्रिया दी यह दिखाना

जिन उपयोगकर्ताओं ने लाइक या प्रतिक्रिया दी है उन्हें उनके usernames और avatars के साथ, सबसे हालिया पहले, प्राप्त करने के लिए `GET` endpoints को `includeUserInfo=public` पास करें। जिन उपयोगकर्ताओं ने अपनी profile activity private कर रखी है वे छोड़ दिए जाते हैं, जैसे anonymous users भी। अधिकतम 100 reacts लौटाए जाते हैं।

SSO users आपका SSO payload `isProfileActivityPrivate` को `false` सेट न करे तो private profile activity के साथ बनाए जाते हैं, इसलिए जिन उपयोगकर्ताओं को आप इन सूचियों में दिखाना चाहते हैं उनके लिए इसे सेट करें।

SSO आवश्यक करने वाले पेजों पर, केवल वे SSO users जो पेज देख सकते हैं, वे ही यह सूचीबद्ध कर सकते हैं कि किसने प्रतिक्रिया दी। अन्य लोगों को `sso-required` या `page-forbidden` कोड के साथ `403` मिलता है।

अपने सर्वर से, [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) और [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) API key के साथ वही सूचियाँ लौटाते हैं, और private profiles वाले उपयोगकर्ताओं को शामिल करने के लिए `includeUserInfo=include-private` भी स्वीकार करते हैं।

### उपयोगकर्ता की पहचान करना

Reacts अनुरोध करने वाले उपयोगकर्ता से जुड़े होते हैं:

- **SSO users:** `sso` query parameter पास करें, जो उसी SSO object के URI encoded JSON पर सेट हो जो आप comment widget को देते हैं। [SSO](/guide-customizations-and-configuration.html#sso) देखें।
- **Anonymous users:** जब `sso` parameter न हो और कोई FastComments लॉगिन न हो, तो सर्वर ब्राउज़र को FastComments session cookie में संग्रहीत एक anonymous id असाइन करता है। अनुरोध `credentials: 'include'` के साथ भेजें ताकि अनुरोधों के बीच cookie बना रहे। जो ब्राउज़र third-party cookies को ब्लॉक करते हैं वे anonymous id को बनाए नहीं रखेंगे, इसलिए जब प्रत्येक उपयोगकर्ता को विश्वसनीय रूप से पहचाना जाना आवश्यक हो तो SSO का उपयोग करें।

### urlId

`urlId` पेज की पहचान करता है, ठीक उसी तरह जैसे यह comments के लिए करता है। वही `urlId` उपयोग करें जो आप comment widget को देते हैं ताकि likes और comments एक ही पेज पर गिने जाएँ। इसे URI encode करना न भूलें।

[inline-code-attrs-start title = 'लाइक बटन उदाहरण'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// Optional, for SSO users. The same object you give the comment widget's "sso" option.
const sso = null;

function likesUrl() {
    let url = `https://fastcomments.com/page-reacts/v1/likes/${tenantId}?urlId=${encodeURIComponent(urlId)}`;
    if (sso) {
        url += '&sso=' + encodeURIComponent(JSON.stringify(sso));
    }
    return url;
}

async function getLikes() {
    const response = await fetch(likesUrl(), {credentials: 'include'});
    return response.json(); // {status, likeCount, didLike, commentCount, urlIdWS}
}

async function like() {
    await fetch(likesUrl(), {method: 'POST', credentials: 'include'});
}

async function unlike() {
    await fetch(likesUrl(), {method: 'DELETE', credentials: 'include'});
}
[inline-code-end]