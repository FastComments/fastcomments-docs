[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

प्रमाणीकरण के लिए, FastComments आपके ब्राउज़र में थर्ड पार्टी कुकीज़ सक्षम होने पर निर्भर करता है। इनके बिना, उपयोगकर्ताओं को कमेंट करने के लिए हमेशा अपना ईमेल छोड़ना होगा (जब तक कि ईमेल इनपुट फ़ील्ड छिपा न हो), और उनके कमेंट हमेशा असत्यापित (डिफ़ॉल्ट रूप से) दिखाई देंगे।

इससे बचने के लिए, आप थर्ड पार्टी कुकी बायपास सक्षम कर सकते हैं।

जब यह सेटिंग सक्षम होती है, तो एक छोटा पॉपअप दिखाई देगा जिसमें एक संदेश दिखेगा कि उपयोगकर्ता लॉग इन हो रहा है। यह पॉपअप तब दिखता है जब भी उपयोगकर्ता कमेंट विजेट के साथ इंटरैक्ट करता है; उदाहरण के लिए, यदि वे कोई कमेंट छोड़ते हैं।

हम कोड में **enableThirdPartyCookieBypass** फ्लैग को true सेट करके ऐसा कर सकते हैं:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'थर्ड-पार्टी कुकी बायपास सक्षम करना'; code-example-end]

हम इसे Widget Customization UI के माध्यम से भी सेट कर सकते हैं, `Enable Third-Party Cookie Popup` के अंतर्गत:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; alt='विजेट कस्टमाइज़ेशन पेज जिसमें Enable Third-Party Cookie Popup चेकबॉक्स चेक किया गया है'; title='थर्ड-पार्टी कुकी बायपास सक्षम करना' app-screenshot-end]