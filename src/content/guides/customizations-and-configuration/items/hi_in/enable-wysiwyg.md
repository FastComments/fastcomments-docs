[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

डिफ़ॉल्ट रूप से, FastComments में फ़ॉर्मेटिंग की सुविधा आपके टेक्स्ट के चारों ओर `<b></b>` जैसे दृश्यमान एंकर टैग जोड़कर की जाती है। टूलबार पर क्लिक करना या शॉर्टकट का उपयोग करना यह आपके लिए कर देता है। हालाँकि, कुछ समुदाय एंकर टैग के बिना फ़ॉर्मेटिंग का उपयोग करना चुन सकते हैं। इसे WYSIWYG (what you see is what you get) एडिटर सक्षम करना कहा जाता है। यह एडिटर डिफ़ॉल्ट वाले जैसा बिल्कुल दिखता है, सिवाय इसके कि यह कुछ अतिरिक्त कोड लोड करता है जो उपयोगकर्ताओं को दृश्यमान एंकर टैग के बिना अपने टेक्स्ट को बोल्ड, अंडरलाइन आदि करने की अनुमति देता है।

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'WYSIWYG एडिटिंग सक्षम करना'; code-example-end]

यह कोड के बिना भी किया जा सकता है। विजेट कस्टमाइज़ेशन पेज में, "Enable Advanced Formatting" विकल्प देखें।

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; alt='विजेट कस्टमाइज़ेशन पेज जिसमें WYSIWYG एडिटर चालू करने के लिए Enable Advanced Formatting चेकबॉक्स चेक किया गया है'; title='WYSIWYG सक्षम करें' app-screenshot-end]