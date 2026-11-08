[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

डिफ़ॉल्ट रूप से, FastComments उपयोगकर्ता से केवल उनकी टिप्पणी, उनका उपयोगकर्ता नाम और उनका ईमेल पूछेगा।

हालाँकि, कुछ स्थितियों में आप चाह सकते हैं कि उपयोगकर्ता अपने ब्लॉग या वेबसाइट का लिंक छोड़ दे।

हम **enableCommenterLinks** फ्लैग को true सेट करके उपयोगकर्ता की वेबसाइट URL छोड़ने के लिए एक अतिरिक्त इनपुट फ़ील्ड दिखाना सक्षम कर सकते हैं:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'कमेंटर लिंक सक्षम करना'; code-example-end]

जब वह URL प्रदान किया जाता है, तो उपयोगकर्ता का खाता अपडेट हो जाएगा और उनका उपयोगकर्ता नाम सभी पिछली और भविष्य की टिप्पणियों में इस URL से लिंक हो जाएगा।

इसे कोड के बिना, विजेट कस्टमाइज़ेशन पेज पर कस्टमाइज़ किया जा सकता है:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; alt='विजेट कस्टमाइज़ेशन पेज जिसमें कमेंटर लिंक चेकबॉक्स चेक किया गया है ताकि कमेंट फ़ॉर्म में वेबसाइट URL फ़ील्ड जोड़ा जा सके'; title='कमेंटर लिंक सक्षम करना' app-screenshot-end]