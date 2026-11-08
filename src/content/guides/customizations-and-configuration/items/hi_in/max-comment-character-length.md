[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

कमेंट इनपुट फ़ील्ड में दर्ज किए जा सकने वाले अधिकतम वर्णों की संख्या **maxCommentCharacterLength** पैरामीटर द्वारा सीमित की जा सकती है।

डिफ़ॉल्ट 2000 है।

इमेज URL जैसी चीज़ें लंबाई निर्धारण में शामिल नहीं होती हैं।

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'कमेंट लंबाई सीमित करें'; code-example-end]

इसे कोड के बिना, विजेट कस्टमाइज़ेशन पेज पर कस्टमाइज़ किया जा सकता है:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; alt='विजेट कस्टमाइज़ेशन पेज पर अधिकतम कमेंट साइज़ फ़ील्ड, जिसका उपयोग यह सीमित करने के लिए किया जाता है कि एक कमेंट में कितने वर्ण हो सकते हैं'; title='कमेंट लंबाई सीमित करें' app-screenshot-end]