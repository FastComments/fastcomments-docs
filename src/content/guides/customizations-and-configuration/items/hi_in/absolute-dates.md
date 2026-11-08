[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

डिफ़ॉल्ट रूप से, स्थानीयकृत सापेक्ष तिथियों का उपयोग किया जाता है। उदाहरण के लिए, हाल ही में छोड़ी गई टिप्पणी के पास आप "11 minutes ago" देख सकते हैं।

पूर्ण तिथियों का उपयोग करना आवश्यक या वांछित हो सकता है, जिस स्थिति में आप इस पैरामीटर को true पर सेट करते हैं।

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'पूर्ण तिथियों का उपयोग करें'; code-example-end]

इसे कोड के बिना, विजेट कस्टमाइज़ेशन पेज पर, Advanced Options के अंतर्गत कस्टमाइज़ किया जा सकता है:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; alt='विजेट कस्टमाइज़ेशन पेज पर Advanced Options, जिसमें absolute dates टॉगल चालू है'; title='पूर्ण तिथियों का उपयोग करें' app-screenshot-end]