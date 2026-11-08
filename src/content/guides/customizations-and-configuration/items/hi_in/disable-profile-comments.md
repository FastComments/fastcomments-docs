[related-parameter-start name = 'disableProfileComments'; type = 'boolean'; related-parameter-end]

डिफ़ॉल्ट रूप से, FastComments उपयोगकर्ता प्रोफाइल पर एक "Profile Comments" टैब दिखाएगा, जिससे विज़िटर किसी के प्रोफाइल पर टिप्पणियाँ छोड़ सकते हैं।

हालाँकि, हम इस टैब को अक्षम कर सकते हैं:

[code-example-start config = {disableProfileComments: true}; linesToHighlight = [6]; title = 'Profile Comments अक्षम करें'; code-example-end]

यह कोड के बिना भी किया जा सकता है। विजेट कस्टमाइज़ेशन पेज पर, "Disable Profile Comments" सेक्शन देखें।

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-comments']; selector = '.disable-profile-comments'; alt='विजेट कस्टमाइज़ेशन पेज जिसमें प्रोफाइल कमेंट्स टैब छिपाने के लिए Disable Profile Comments चेकबॉक्स चेक किया गया है'; title='Profile Comments अक्षम करें' app-screenshot-end]