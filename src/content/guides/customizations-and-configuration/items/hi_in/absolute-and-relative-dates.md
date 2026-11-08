[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

डिफ़ॉल्ट रूप से, स्थानीयकृत सापेक्ष तिथियों का उपयोग किया जाता है। उदाहरण के लिए, हाल ही में छोड़ी गई टिप्पणी के आगे आप "11 minutes ago" देख सकते हैं।

यह आवश्यक या वांछित हो सकता है कि इस सापेक्ष तिथि प्रारूप को बनाए रखा जाए, लेकिन साथ ही उसके आगे पूरी तिथि भी दिखाई जाए, जिस स्थिति में आप इस पैरामीटर को true पर सेट करते हैं।

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'पूर्ण और सापेक्ष दोनों तिथियों का उपयोग करें'; code-example-end]

इसे कोड के बिना, विजेट कस्टमाइज़ेशन पेज पर, Advanced Options के अंतर्गत कस्टमाइज़ किया जा सकता है। इस विकल्प को UI में देखने के लिए आपको पहले Absolute Dates सक्षम करना होगा।

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; alt='विजेट कस्टमाइज़ेशन पेज पर Advanced Options, जिसमें पूर्ण तिथियाँ और संयुक्त सापेक्ष तिथि सेटिंग दोनों सक्षम हैं'; title='पूर्ण और सापेक्ष दोनों तिथियों का उपयोग करें' app-screenshot-end]