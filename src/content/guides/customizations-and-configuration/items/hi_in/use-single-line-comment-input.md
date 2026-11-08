[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

डिफ़ॉल्ट रूप से, FastComments उपयोगकर्ता को डिफ़ॉल्ट वर्ण सीमा तक जितनी चाहें उतनी पंक्तियों का कमेंट दर्ज करने की अनुमति देता है।

हालाँकि, उपयोगकर्ता को केवल एक ही पंक्ति का टेक्स्ट दर्ज करने तक सीमित करना बेहतर हो सकता है। कुछ उदाहरण उपयोग के मामलों में ऑनलाइन बिडिंग या लाइव चैट शामिल हैं, जिनके लिए FastComments का उपयोग किया जा सकता है।

हम **useSingleLineCommentInput** फ्लैग को इस प्रकार सक्षम करते हैं:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'सिंगल-लाइन कमेंट इनपुट सक्षम करें'; code-example-end]

यह कोड के बिना भी किया जा सकता है। विजेट कस्टमाइज़ेशन पेज में, "Enable Single-Line Comment Input" सेक्शन देखें।

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; alt='विजेट कस्टमाइज़ेशन पेज में सिंगल-लाइन कमेंट इनपुट चेकबॉक्स चालू है, जो इनपुट को एक पंक्ति तक सीमित करता है'; title='सिंगल-लाइन कमेंट इनपुट सक्षम करें' app-screenshot-end]

ध्यान दें कि, प्रत्येक सॉर्ट दिशा के लिए प्रत्येक पेज पर कमेंट्स पहले से गणना की जाती हैं, इसलिए सभी सॉर्ट दिशाओं में प्रदर्शन समान होता है।