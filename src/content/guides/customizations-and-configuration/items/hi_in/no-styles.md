[related-parameter-start name = 'noStyles'; type = 'boolean'; related-parameter-end]

बड़े कस्टम स्टाइलिंग प्रोजेक्ट्स के लिए, यह बेहतर हो सकता है कि आप एक साफ़ स्लेट से शुरुआत करें और डिफ़ॉल्ट स्टाइलिंग का बिल्कुल उपयोग न करें।

सभी डिफ़ॉल्ट स्टाइलिंग को **noStyles** पैरामीटर को true पर सेट करके हटाया जा सकता है, जैसा कि नीचे दिखाया गया है:

[code-example-start config = {noStyles: true}; linesToHighlight = [6]; title = 'सभी डिफ़ॉल्ट स्टाइल्स अक्षम करना'; code-example-end]

इसे कोड के बिना, विजेट कस्टमाइज़ेशन पेज पर, Advanced Options के अंतर्गत कस्टमाइज़ किया जा सकता है:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.disable-all-default-styling']; selector = '.disable-all-default-styling'; alt='विजेट कस्टमाइज़ेशन पेज पर Advanced Options के अंतर्गत Disable all default styling चेकबॉक्स सक्रिय'; title='सभी डिफ़ॉल्ट स्टाइल्स अक्षम करना' app-screenshot-end]