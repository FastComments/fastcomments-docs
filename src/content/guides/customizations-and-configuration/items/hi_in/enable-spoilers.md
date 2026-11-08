[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

हम **enableSpoilers** फ्लैग को true सेट करके स्पॉइलर सपोर्ट सक्षम कर सकते हैं:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'स्पॉइलर सक्षम करना'; code-example-end]

यह कोड के बिना भी किया जा सकता है। विजेट कस्टमाइज़ेशन पेज में, "Enable Spoilers" विकल्प देखें।

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; alt='विजेट कस्टमाइज़ेशन पेज जिसमें Enable Spoilers चेकबॉक्स चेक किया गया है ताकि एडिटर में SPOILER बटन जुड़ जाए'; title='स्पॉइलर सक्षम करें' app-screenshot-end]

जब टेक्स्ट हाइलाइट किया जाता है और अब दिखाई दे रहे `SPOILER` बटन पर क्लिक किया जाता है, तो टेक्स्ट तब तक मास्क (छिपा) रहेगा जब तक उपयोगकर्ता उस पर माउस न ले जाए। डार्क मोड के लिए हम वही काम करते हैं, लेकिन अलग रंगों के साथ जो डार्क मोड से बेहतर मेल खाते हैं।

यह WYSIWYG एडिटर के साथ भी संगत है।