[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

डिफ़ॉल्ट रूप से, FastComments कमेंट एरिया के ऊपरी दाएँ कोने में एक नोटिफिकेशन बेल दिखाएगा।

यह बेल लाल हो जाएगी और उपयोगकर्ता के पास मौजूद नोटिफिकेशन की संख्या दिखाएगी। नोटिफिकेशन के कुछ उदाहरण हैं:

- किसी उपयोगकर्ता ने आपको उत्तर दिया।
- किसी उपयोगकर्ता ने उस थ्रेड में उत्तर दिया जिसमें आपने कमेंट किया था।
- किसी उपयोगकर्ता ने आपके कमेंट को अप-वोट किया।
- किसी उपयोगकर्ता ने उस पेज पर उत्तर दिया जिसकी आपने सदस्यता ली है।

नोटिफिकेशन बेल पूरे पेज की सदस्यता लेने का एक तरीका भी प्रदान करती है।

हालाँकि, हम नोटिफिकेशन बेल को पूरी तरह से अक्षम कर सकते हैं:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'नोटिफिकेशन बेल अक्षम करें'; code-example-end]

यह कोड के बिना भी किया जा सकता है। विजेट कस्टमाइज़ेशन पेज पर, "Disable Notification Bell" सेक्शन देखें।

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; alt='विजेट कस्टमाइज़ेशन पेज जिसमें Disable Notification Bell चेकबॉक्स चेक किया गया है'; title='नोटिफिकेशन बेल अक्षम करें' app-screenshot-end]