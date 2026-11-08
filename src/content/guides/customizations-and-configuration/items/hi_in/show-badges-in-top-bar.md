[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

डिफ़ॉल्ट रूप से, FastComments यूज़र बैज केवल कमेंट थ्रेड के भीतर उनकी टिप्पणियों पर प्रदर्शित करता है।

हालाँकि, हम विजेट कस्टमाइज़ेशन पेज में इस सुविधा को सक्षम करके कमेंट फ़ॉर्म के ऊपर उनके नाम के बगल में यूज़र बैज दिखा सकते हैं:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; alt='विजेट कस्टमाइज़ेशन पेज पर Show badges in top bar चेकबॉक्स, जो कमेंट फ़ॉर्म के ऊपर नाम के बगल में बैज दिखाता है'; title='Show Badges in Top Bar विकल्प' app-screenshot-end]

यह टॉप बार क्षेत्र में यूज़र के बैज को उनके नाम के साथ प्रदर्शित करेगा, जिससे कमेंट लिखते समय उनकी उपलब्धियाँ और स्थिति अधिक प्रमुख होगी।

ध्यान दें कि यह सुविधा काम करने के लिए विजेट कस्टमाइज़ेशन UI में सक्षम होनी चाहिए। आप वैकल्पिक रूप से अपने कोड कॉन्फ़िगरेशन में **showBadgesInTopBar** फ़्लैग को false पर सेट करके इसे चुनिंदा रूप से अक्षम कर सकते हैं, भले ही यह सर्वर स्तर पर चालू हो:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Show Badges in Top Bar अक्षम करें'; code-example-end]