कमेंट बॉक्स के चारों ओर का बॉर्डर टेक्स्ट एरिया के अपने बॉर्डर और विजेट द्वारा उसके चारों ओर बनाई गई कुछ पतली लाइनों से मिलकर बना है। इसका रंग या कोनों की गोलाई बदलने के लिए, `textarea` को सीधे स्टाइल करने के बजाय इन CSS वेरिएबल को सेट करें। ये वेरिएबल बॉर्डर के हर हिस्से को एक साथ रीस्टाइल करते हैं, इसलिए कोने और रंग हमेशा आपस में मेल खाते हैं।

| Variable | What it changes | Default |
|---|---|---|
| `--fc-input-border-color` | बॉर्डर का रंग | `#bfbfbf` |
| `--fc-input-border-color-focus` | जब यूज़र टाइप कर रहा हो तब बॉर्डर का रंग | `#555` |
| `--fc-input-border-radius` | गोल कोनों की गोलाई | `11px` |
| `--fc-input-border-start-start-radius` | चौकोर ऊपरी-बाएँ कोने की गोलाई (दाएँ-से-बाएँ भाषाओं में ऊपरी-दायाँ) | `0` |

CSS को [Widget Customization page](https://fastcomments.com/auth/my-account/customize-widget) पर **Custom CSS** बॉक्स में जोड़ें, या इसे `customCSS` विकल्प के साथ पास करें। आपको केवल उन्हीं वेरिएबल को सेट करने की आवश्यकता है जिन्हें आप बदलना चाहते हैं।

आप Custom CSS बॉक्स के ठीक नीचे मौजूद **Comment box border** हेल्पर का भी उपयोग कर सकते हैं, जो यह CSS आपके लिए लिख देता है।

## बॉर्डर का रंग बदलें

[inline-code-attrs-start title = 'बॉर्डर का रंग'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## टाइप करते समय बॉर्डर का रंग बदलें

[inline-code-attrs-start title = 'टाइप करते समय बॉर्डर का रंग'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## चारों कोनों को गोल करें

डिफ़ॉल्ट रूप से ऊपरी-बायाँ कोना चौकोर होता है। चारों कोनों को समान रूप से गोल करने के लिए दोनों रेडियस वेरिएबल सेट करें:

[inline-code-attrs-start title = 'चारों कोनों को गोल करें'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## चौकोर कोने

[inline-code-attrs-start title = 'चौकोर कोने'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## अपने ब्रांड से मिलाएँ

[inline-code-attrs-start title = 'ब्रांड के रंग और कोने'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## डार्क मोड में अलग रंग

जब विजेट डार्क मोड में होता है तो उसमें `dark` क्लास होती है, इसलिए आप डार्क मोड के लिए अलग मान सेट कर सकते हैं:

[inline-code-attrs-start title = 'डार्क मोड बॉर्डर रंग'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## टेक्स्ट एरिया को सीधे क्यों न स्टाइल करें?

विजेट कमेंट बॉक्स के बॉर्डर का एक हिस्सा स्वयं बनाता है, टेक्स्ट एरिया के चारों ओर। यदि आप केवल `textarea` पर `border-color` या `border-radius` सेट करते हैं, तो वे लाइनें डिफ़ॉल्ट स्टाइल में ही रहती हैं और बॉर्डर बेमेल दिखता है, उदाहरण के लिए गोल कोने से गुज़रती एक चौकोर लाइन। ऊपर दिए गए वेरिएबल दोनों को एक साथ बदल देते हैं।