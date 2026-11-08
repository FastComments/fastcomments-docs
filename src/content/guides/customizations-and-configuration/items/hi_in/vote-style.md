[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

डिफ़ॉल्ट रूप से, FastComments वोटिंग विकल्पों को ऊपर और नीचे तीरों के रूप में प्रदर्शित करता है, जिससे उपयोगकर्ता किसी कमेंट को ऊपर या नीचे वोट कर सकते हैं।

हालाँकि, वोट टूलबार की शैली को बदलना संभव है। वर्तमान विकल्प डिफ़ॉल्ट Up/Down बटन हैं, या Heart शैली की वोटिंग प्रणाली का उपयोग करना।

हम **voteStyle** फ्लैग का उपयोग इस प्रकार करते हैं:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Heart बटन सक्षम करें'; code-example-end]

हम दृढ़ता से सुझाव देते हैं कि आप इसे कोड के बिना करें क्योंकि इससे सर्वर-साइड वैलिडेशन भी सक्षम होती है। विजेट कस्टमाइज़ेशन पेज में, "Vote Style" सेक्शन देखें।

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; alt='विजेट कस्टमाइज़ेशन पेज पर Vote Style सेटिंग, ऊपर-नीचे तीर या heart वोटिंग के विकल्प प्रदान करती है'; title='वोटिंग शैली बदलें' app-screenshot-end]

वोटिंग को अक्षम भी किया जा सकता है, शैली विकल्पों के ऊपर `Disable Voting` देखें।