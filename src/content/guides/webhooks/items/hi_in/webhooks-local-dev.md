लोकल डेवलपमेंट के लिए, [ngrok](https://ngrok.com/) जैसे टूल का उपयोग करें।

सिस्टम को सुरक्षित रखना आसान बनाने के लिए, लोकल डेवलपमेंट अन्य एनवायरनमेंट सेट अप और सुरक्षित करने की उसी प्रक्रिया का पालन करता है।

### चरण 1: अपने खाते के डोमेन में "localhost" जोड़ें।

"localhost" को [यहाँ एक डोमेन के रूप में जोड़ें](https://fastcomments.com/auth/my-account/configure-domains)।

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='खाता सेटिंग्स में डोमेन जोड़ने का फॉर्म जिसमें डोमेन नाम फ़ील्ड में localhost दर्ज है'; title='localhost जोड़ें'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### चरण 2: एक API Key चुनें

हम आपके डोमेन के लिए वेबहुक कॉन्फ़िगरेशन जोड़ने जा रहे हैं, इसलिए हमें एक API key की आवश्यकता होगी। [आप यह यहाँ कर सकते हैं।](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='नया API secret फॉर्म जिसमें संबद्ध डोमेन localhost पर सेट है और key का नाम Testing है'; title='Testing API Key जोड़ें'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

"Associate with domain" के अंतर्गत - अपना "localhost" डोमेन चुनें।

**ध्यान दें: वैकल्पिक रूप से, आप सभी टेस्टिंग गतिविधि और स्टेजिंग एनवायरनमेंट के लिए एक ही API Secret का उपयोग कर सकते हैं। बस "All Domains" के लिए एक API Secret जोड़ें, और इसे "test" जैसा नाम दें।**

सुनिश्चित करें कि आपके प्रोडक्शन डोमेन(s) के लिए एक API Secret परिभाषित है। अन्य सभी डोमेन के इवेंट्स वाइल्डकार्ड (टेस्टिंग) secret का उपयोग करेंगे।

### चरण 3: अपना वेबहुक जोड़ें

ngrok या समान टूल चलाते समय, "localhost" के लिए मान [यहाँ](https://fastcomments.com/auth/my-account/manage-data/webhooks) सेट करें।

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; alt='वेबहुक्स एडमिन जिसमें localhost डोमेन चयनित है और comment created एंडपॉइंट में एक ngrok URL भरा हुआ है'; title='Testing वेबहुक जोड़ें'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

`Send Test Payload` पर क्लिक करने पर, हम यह जांचने के लिए दो टेस्ट इवेंट्स भेजेंगे कि आप API key को वैलिडेट करते हैं या नहीं।

एक बार वैलिडेट होने के बाद, `Save` दबाएं।

### चरण 4: एक कमेंट जोड़ें

अब आप कमेंट्स जोड़, संपादित या हटा सकते हैं और आपको हमें अपनी टेस्टिंग API key का उपयोग करके इवेंट्स के साथ आपकी लोकल डेवलपमेंट मशीन को कॉल करते देखना चाहिए। इवेंट्स के आपकी मशीन तक पहुंचने में 30 सेकंड तक की देरी हो सकती है।