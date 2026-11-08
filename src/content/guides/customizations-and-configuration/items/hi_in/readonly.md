[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

readonly फ्लैग को true सेट करके कमेंटिंग को लॉक किया जा सकता है ताकि कोई नए कमेंट या वोट न दिए जा सकें।

कमेंट्स को एडिट या डिलीट भी नहीं किया जा सकेगा।

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'कमेंट थ्रेड को Readonly बनाना'; code-example-end]

इसे कोड के बिना, विजेट कस्टमाइज़ेशन पेज पर, पूरे डोमेन या पेज के लिए कस्टमाइज़ किया जा सकता है:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; alt='विजेट कस्टमाइज़ेशन पेज पर Prevent new replies सेटिंग, जो डोमेन या पेज के लिए थ्रेड को लॉक करती है'; title='कमेंट थ्रेड को Readonly बनाना' app-screenshot-end]

## अपडेट!

नवंबर 2022 से, एडमिनिस्ट्रेटर और मॉडरेटर रिप्लाई एरिया के ऊपर तीन-डॉट मेन्यू के ज़रिए थ्रेड्स को **लाइव** लॉक या अनलॉक कर सकते हैं।

इससे नए कमेंट्स रोके जाएंगे, लेकिन वोटिंग की अनुमति बनी रहेगी, और उपयोगकर्ता चाहें तो अपने कमेंट्स डिलीट कर सकेंगे, जबकि `readonly` इन चीज़ों की अनुमति नहीं देता।

यह `Page` API में `isClosed` फ़ील्ड से मेल खाता है।