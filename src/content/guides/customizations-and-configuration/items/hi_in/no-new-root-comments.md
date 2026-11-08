[related-parameter-start name = 'noNewRootComments'; type = 'boolean'; related-parameter-end]

`noNewRootComments` को `true` पर सेट करने से विजेट रूट रिप्लाई एरिया को छिपा देगा, लेकिन उपयोगकर्ताओं को अभी भी चाइल्ड कमेंट्स का जवाब देने की अनुमति रहेगी। उदाहरण के लिए, आप इसे पेज लोड पर शर्त के आधार पर सेट कर सकते हैं ताकि केवल कुछ उपयोगकर्ता ही टॉप-लेवल कमेंट कर सकें।

[code-example-start config = {noNewRootComments: true}; linesToHighlight = [6]; title = 'नए रूट कमेंट्स को रोकें'; code-example-end]