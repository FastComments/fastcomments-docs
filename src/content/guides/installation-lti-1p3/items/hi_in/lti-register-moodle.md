**Moodle का उपयोग कर रहे हैं?** हम FastComments के लिए एक समर्पित Moodle प्लगइन भी प्रकाशित करते हैं, जो LTI 1.3 से अधिक गहरा एकीकरण प्रदान करता है (grade sync hooks, गहरी activity reporting, नेटिव Moodle सेटिंग्स UI)। देखें <a href="/guide-installation-moodle.html" target="_blank">Moodle प्लगइन इंस्टॉलेशन गाइड</a>। नीचे दिया गया LTI 1.3 फ्लो सही विकल्प है यदि आप एक ऐसी single registration चाहते हैं जो अन्य LMSes को भी कवर करे, या यदि आपका Moodle एडमिन थर्ड-पार्टी प्लगइन इंस्टॉल नहीं करेगा।

Moodle 4.0+ External Tool प्लगइन के माध्यम से LTI 1.3 Dynamic Registration का समर्थन करता है।

#### Tool Management स्क्रीन खोलें

1. Moodle में साइट एडमिनिस्ट्रेटर के रूप में साइन इन करें।
2. **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** पर जाएं।

#### URL पेस्ट करें

आपको **Tool URL** नाम का एक कार्ड दिखाई देगा। FastComments registration URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">यहाँ से प्राप्त करें</a>) टेक्स्ट फ़ील्ड में पेस्ट करें और **Add LTI Advantage** पर क्लिक करें।

Moodle एक registration स्क्रीन खोलता है जो टूल की पहचान और उसके द्वारा अनुरोधित अनुमतियाँ दिखाती है। समीक्षा करें और **Activate** (या Moodle संस्करण के अनुसार **Register**) पर क्लिक करें।

registration पूरी होने पर पॉपअप बंद हो जाता है; नया FastComments टूल **Tools** सूची में **Active** स्थिति के साथ दिखाई देता है।

#### इसे उपलब्ध कराएं

डिफ़ॉल्ट रूप से Moodle नए टूल्स को "Course tools" सूची में जोड़ता है लेकिन उन्हें activity picker में नहीं दिखाता है। FastComments को पूरे कोर्स में उपलब्ध कराने के लिए:

1. FastComments टाइल पर गियर आइकन पर क्लिक करें।
2. **Tool configuration usage** के अंतर्गत, **Show in activity chooser and as a preconfigured tool** चुनें।
3. सहेजें।

अब इंस्ट्रक्टर्स किसी भी कोर्स में **Add an activity or resource** > **FastComments** के माध्यम से FastComments जोड़ सकते हैं।