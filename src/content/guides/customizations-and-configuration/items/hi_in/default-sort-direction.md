[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

डिफ़ॉल्ट रूप से, FastComments टिप्पणियों को "Most Relevant" सॉर्ट दिशा के अनुसार क्रमबद्ध करता है।

Most Relevant सॉर्टिंग टिप्पणी करने के समय और वोटों की संख्या को ध्यान में रखकर क्रमबद्ध करती है।

उपयोगकर्ता तब कमेंट विजेट UI में सॉर्ट दिशा को Oldest या Newest First में बदल सकता है।

हालाँकि, हम डिफ़ॉल्ट को इन तीनों में से किसी भी विकल्प में बदल सकते हैं। उदाहरण के लिए, यदि आप सबसे पुरानी टिप्पणियाँ पहले दिखाना चाहते हैं:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'डिफ़ॉल्ट सॉर्ट को Oldest First में बदलना'; code-example-end]

हमने **defaultSortDirection** का मान "OF" सेट किया ताकि दिशा "OF" हो जाए।

Newest-first सॉर्ट दिशा के लिए, हम निम्नलिखित करेंगे:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'डिफ़ॉल्ट सॉर्ट को Newest First में बदलना'; code-example-end]

**defaultSortDirection** के लिए मान्य मान हैं:

- MR: "Most Recent"
- NF: "Newest First"
- OF: "Oldest First"

यह कोड के बिना भी किया जा सकता है। विजेट कस्टमाइज़ेशन पेज में, "Default Sort Direction" सेक्शन देखें।

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; alt='Default Sort Direction सेलेक्टर जो Most Relevant, Newest First, और Oldest First विकल्प देता है'; title='डिफ़ॉल्ट सॉर्ट दिशा बदलना' app-screenshot-end]

ध्यान दें कि, प्रत्येक सॉर्ट दिशा के लिए प्रत्येक पेज की टिप्पणियाँ पूर्व-गणना (pre-computed) की जाती हैं, इसलिए सभी सॉर्ट दिशाओं का प्रदर्शन समान होता है।