[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

कमेंट विजेट के शीर्ष पर दिखाया गया कमेंट काउंट कस्टमाइज़ किया जा सकता है।

इसे किसी भी स्ट्रिंग से बदला जा सकता है, और **[count]** मान को काउंट मान से बदल दिया जाएगा, जो उपयोगकर्ता के लिए स्थानीयकृत (localized) होता है।

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = 'कमेंट काउंट टेक्स्ट को कस्टमाइज़ करना'; code-example-end]

इसे कोड के बिना, विजेट कस्टमाइज़ेशन पेज पर कस्टमाइज़ किया जा सकता है:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; alt='विजेट कस्टमाइज़ेशन पेज पर कमेंट काउंट टेक्स्ट फ़ील्ड, जहाँ [count] को लाइव कुल संख्या से बदल दिया जाता है'; title='कमेंट काउंट टेक्स्ट को कस्टमाइज़ करना' app-screenshot-end]