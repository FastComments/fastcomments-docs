यदि आप अपनी साइट को WordPress से हटाकर नई साइट पर FastComments लगाना चाहते हैं, तो आपको WordPress प्लगइन की आवश्यकता नहीं है। WordPress से अपनी कमेंट्स एक्सपोर्ट करें, फिर FastComments डैशबोर्ड के [Import page](https://fastcomments.com/auth/my-account/manage-data/import) पर फ़ाइल अपलोड करें।

हम दो WordPress एक्सपोर्ट फ़ॉर्मैट सपोर्ट करते हैं।

### WordPress XML (अनुशंसित)

यह WordPress के बिल्ट-इन एक्सपोर्टर से मिलने वाली फ़ाइल है, इसलिए किसी अतिरिक्त प्लगइन की ज़रूरत नहीं है।

1. अपने WordPress एडमिन में, `Tools -> Export` पर जाएँ।
2. `All content` चुनें और `Download Export File` पर क्लिक करें।
3. FastComments के [Import page](https://fastcomments.com/auth/my-account/manage-data/import) पर, `WordPress (.xml)` चुनें और फ़ाइल अपलोड करें।

प्रत्येक कमेंट उस पोस्ट के URL से जुड़ा होता है जिस पर वह लिखा गया था, और यह URL पहले से फ़ाइल में मौजूद होता है।

इम्पोर्ट लेखक का नाम, ईमेल और वेबसाइट, तारीख, कंटेंट, रिप्लाई थ्रेडिंग, और कमेंट अप्रूव हुआ या नहीं — यह सब सुरक्षित रखता है। कमेंटर के अवतार Gravatar से लाए जाते हैं। वोट्स इस फ़ॉर्मैट का हिस्सा नहीं होते।

### WordPress CSV

यह [WebToffee's WordPress Comments Import & Export plugin](https://wordpress.org/plugins/comments-import-export-woocommerce/) से मिलने वाली फ़ाइल है।

1. अपने WordPress एडमिन में प्लगइन इंस्टॉल करें और अपनी कमेंट्स को CSV के रूप में एक्सपोर्ट करें।
2. प्रत्येक `comment_post_ID` मान को पोस्ट के URL से बदलें।
3. FastComments के [Import page](https://fastcomments.com/auth/my-account/manage-data/import) पर, `WordPress (.csv)` चुनें और फ़ाइल अपलोड करें।

प्रत्येक कमेंट `comment_post_ID` कॉलम से जुड़ा होता है। WordPress इस कॉलम में पोस्ट ID भरता है, और आपकी नई साइट पर WordPress पोस्ट IDs नहीं होते, इसलिए चरण 2 में इसे URL से बदला जाता है।

इम्पोर्ट लेखक का नाम, ईमेल और वेबसाइट, तारीख, कंटेंट, रिप्लाई थ्रेडिंग, और कमेंट अप्रूव हुआ या नहीं — यह सब सुरक्षित रखता है। कमेंटर के अवतार Gravatar से लाए जाते हैं। यह WordPress का स्पैम फ़्लैग भी सुरक्षित रखता है, और फ़ाइल में शामिल होने पर wpDiscuz के लाइक और डिसलाइक भी।

### नए पेजों से कमेंट्स का मिलान

यदि आपकी नई साइट के URLs आपकी WordPress साइट जैसे ही हैं, तो कमेंट्स बिना किसी अतिरिक्त सेटअप के संबंधित पेजों पर दिखाई देंगे।

यदि डोमेन बदलता है, तो इम्पोर्ट के बाद [Domain Migration tool](/guide-migrations.html#migrating-domains) चलाएँ। यदि अलग-अलग पेज URLs बदलते हैं, तो आप प्रत्येक पेज को उसके पुराने URL से नए URL पर [migrate each page](/guide-migrations.html#migrating-pages) कर सकते हैं।

बड़े पैमाने पर पेज माइग्रेशन के लिए, जैसे कमेंट विजेट के [urlId](/guide-customizations-and-configuration.html#url-id) फ़ील्ड में पास किए गए मान से डोमेन हटाना, [open a support ticket](https://fastcomments.com/auth/my-account/help) करें और हम इसे आपके लिए संभाल लेंगे।

### स्विच करने से पहले

आप इम्पोर्ट जितनी बार चाहें चला सकते हैं। एक ही फ़ाइल को दोबारा इम्पोर्ट करने से [डुप्लिकेट नहीं बनते](/guide-migrations.html#importing-data), इसलिए आप नई साइट टेस्ट करने के लिए एक बार इम्पोर्ट कर सकते हैं, और फिर स्विच करने से ठीक पहले अपनी नवीनतम कमेंट्स के साथ दोबारा इम्पोर्ट कर सकते हैं।

1GB से बड़ी एक्सपोर्ट फ़ाइलों के लिए, [सपोर्ट से संपर्क करें](https://fastcomments.com/auth/my-account/help)।

अपनी नई साइट पर FastComments जोड़ने के लिए, [Installation guide](/guide-installation.html) देखें।