[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

डिफ़ॉल्ट रूप से, टॉप लेवल कमेंट्स के जवाब दिखाए जाते हैं।

इसे इस तरह कॉन्फ़िगर किया जा सकता है कि उपयोगकर्ता को चाइल्ड कमेंट्स देखने के लिए टॉप-लेवल कमेंट्स पर "Show Replies" पर क्लिक करना पड़े।

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'टॉप लेवल कमेंट्स के जवाब छिपाएँ'; code-example-end]

इसे कोड के बिना, विजेट कस्टमाइज़ेशन पेज पर कस्टमाइज़ किया जा सकता है:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; alt='विजेट कस्टमाइज़ेशन UI में collapse replies विकल्प, जो चाइल्ड कमेंट्स को Show Replies लिंक के पीछे छिपाता है'; title='जवाब छिपाएँ' app-screenshot-end]

यह सेटिंग शुरू में लोड किए गए टॉप लेवल कमेंट्स की संख्या को प्रभावित नहीं करेगी। यदि आपके पास एक टॉप लेवल कमेंट और 29 चाइल्ड कमेंट्स हैं, तो इस सेटिंग के चालू होने पर आप:

- टॉप लेवल कमेंट देखेंगे।
- इस कमेंट के नीचे Show Replies (29) देखेंगे।

यदि आप इस विकल्प के साथ सभी टॉप लेवल कमेंट्स दिखाना चाहते हैं, तो [starting page को -1 पर सेट करें](#starting-page)।