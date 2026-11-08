अब जब हमने zip फ़ाइल डाउनलोड कर ली है, इसे एक फ़ोल्डर में एक्सट्रैक्ट करें। मैंने डिफ़ॉल्ट `casper.zip` डाउनलोड किया और Windows पर `Downloads\casper` में एक्सट्रैक्ट किया।

इसके बाद, यह सुनिश्चित करें कि आपके पास NodeJS का LTS या नया संस्करण इंस्टॉल है। आप इसे यहाँ से प्राप्त कर सकते हैं: https://nodejs.org/en/download/

NodeJS इंस्टॉल होने के बाद, आपको एक कोड एडिटर इंस्टॉल करना चाहिए।

हम Webstorm की सिफारिश करते हैं (और उसी का उपयोग करते हैं), जिसे आप यहाँ से 30-दिन के ट्रायल के साथ प्राप्त कर सकते हैं (क्रेडिट कार्ड की आवश्यकता नहीं): https://www.jetbrains.com/webstorm/

इसके बाद सबसे अच्छा मुफ्त विकल्प संभवतः Visual Studio Code है: https://code.visualstudio.com/download

एक बार जब आपका एडिटर सेटअप हो जाए और थीम फ़ोल्डर एडिटर में खुल जाए, तो IDE में टर्मिनल खोलें और चलाएँ:

[inline-code-attrs-start title = 'थीम इंस्टॉल करें'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

सफल आउटपुट इस तरह दिखेगा (आप चेतावनियों को अनदेखा कर सकते हैं):

<div class="screenshot white-bg">
    <div class="title">सफल npm install आउटपुट</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="सफल npm install आउटपुट" />
</div>

यह उन बाद के कमांड्स के लिए थीम की डिपेंडेंसीज़ सेट अप करेगा जिन्हें हम चलाएँगे। साथ ही, एक्सपोर्ट थीम की डिपेंडेंसीज़ के इंस्टॉल होने पर निर्भर करता है, अन्यथा री-इंपोर्ट ठीक से काम नहीं करेगा।