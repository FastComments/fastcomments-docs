[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

डिफ़ॉल्ट रूप से, FastComments पेज पर उपयोगकर्ताओं की सूची नहीं दिखाता है।

आप कमेंट विजेट के साथ, उन लोगों की सूची दिखा सकते हैं जो इस समय पेज देख रहे हैं। यह सूची उपयोगकर्ताओं के जुड़ने और छोड़ने पर लाइव अपडेट होती है, और उनका नाम, अवतार और ऑनलाइन संकेतक दिखाती है।

तीन लेआउट विकल्प हैं:

- `1` - ऊपर: कमेंट्स के ऊपर रेंडर किया गया ओवरलैपिंग अवतारों की एक क्षैतिज पंक्ति।
- `2` - बाएँ: विजेट के बाईं ओर रेंडर किया गया नामों और ऑनलाइन डॉट्स वाला साइडबार।
- `3` - दाएँ: वही साइडबार विजेट के दाईं ओर रेंडर किया गया।

इस सुविधा को सक्षम करने के लिए **usersListLocation** फ्लैग सेट करें:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'दाईं ओर उपयोगकर्ता सूची दिखाएँ'; code-example-end]

डिफ़ॉल्ट रूप से सूची में केवल वर्तमान में ऑनलाइन उपयोगकर्ता दिखते हैं। उन लोगों को भी शामिल करने के लिए जिन्होंने पहले पेज पर कमेंट किया है (लेकिन अभी इसे देख नहीं रहे हैं), **usersListIncludeOffline** को true सेट करें:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'पिछले कमेंटर्स शामिल करें'; code-example-end]

पिछले कमेंटर्स हरे ऑनलाइन डॉट के बिना रेंडर किए जाते हैं ताकि यह स्पष्ट रहे कि इस समय कौन मौजूद है।

प्राइवेट प्रोफ़ाइल वाले उपयोगकर्ताओं को एक सामान्य अवतार और "Private Profile" लेबल के साथ दिखाया जाता है ताकि पहचान उजागर किए बिना गिनती सटीक रहे।

इसे कोड के बिना भी कॉन्फ़िगर किया जा सकता है। विजेट कस्टमाइज़ेशन पेज पर, "Users List Location" विकल्प देखें। जब लोकेशन Off के अलावा किसी अन्य मान पर सेट हो, तो उसके नीचे एक "Include past commenters" चेकबॉक्स दिखाई देता है।

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; alt='Users List Location दाईं ओर सेट है, और उसके नीचे include past commenters चेकबॉक्स दिखाई दे रहा है'; title='उपयोगकर्ता सूची सेटिंग्स'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

500 लाइव उपयोगकर्ताओं से अधिक होने पर, सूची 30 सेकंड तक पुरानी हो सकती है।