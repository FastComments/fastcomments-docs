चार सार्वजनिक vals जिन्हें आप remix कर सकते हैं, जिनमें से प्रत्येक इस गाइड के एक हिस्से को कवर करता है।

**[Blog with comments](https://www.val.town/x/fastcomments/blog-with-comments)** ([live](https://fastcomments-blog.val.run)) एक Markdown ब्लॉग है जिसमें हर पोस्ट के नीचे एक थ्रेड और इंडेक्स पर बल्क कमेंट काउंट हैं। यह remix करते ही काम करने लगता है, और एक environment variable इसे आपके अपने खाते से जोड़ देता है।

**[SSO demo](https://www.val.town/x/fastcomments/sso-demo)** ([live](https://fastcomments-sso.val.run)) विज़िटर को उनके Val Town खाते से साइन इन करता है और उस पहचान को विजेट को सौंप देता है, ताकि दूसरा लॉगिन करने की ज़रूरत न पड़े।

**[Webhook receiver](https://www.val.town/x/fastcomments/webhook-receiver)** ([live](https://fastcomments-webhooks.val.run)) हर डिलीवरी पर HMAC सिग्नेचर को सत्यापित करता है और इवेंट्स को SQLite में सहेजता है। इसमें एक बटन है जो एक टेस्ट पेलोड पर सिग्न करके उसे स्वयं को डिलीवर करता है, ताकि आप असली webhook कॉन्फ़िगर करने से पहले सत्यापन सफल होते देख सकें।

**[Agent skills](https://www.val.town/x/fastcomments/skills)** ([live](https://fastcomments-skills.val.run)) FastComments एजेंट स्किल्स की एक लाइब्रेरी है जो विजेट, SSO, REST API, मॉडरेशन, और Disqus से माइग्रेशन को कवर करती है। इसे remix करें और Val Town का एजेंट, Townie, `skills/` से स्किल्स को स्वतः उठा लेता है, ताकि आपका एजेंट बिना डॉक्यूमेंटेशन चैट में पेस्ट किए कमेंट्स सेट अप करना जानता हो।

वही स्किल्स कहीं और भी `npx skills add fastcomments/skills` के साथ इंस्टॉल हो जाती हैं।