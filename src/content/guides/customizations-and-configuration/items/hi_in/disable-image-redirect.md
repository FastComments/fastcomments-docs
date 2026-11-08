[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

डिफ़ॉल्ट रूप से, FastComments उपयोगकर्ताओं को इमेज अपलोड करने की अनुमति देता है। जब कोई उपयोगकर्ता उस इमेज पर क्लिक करता है, तो FastComments डिफ़ॉल्ट रूप से
उस इमेज को पूरी तरह दिखाने के लिए एक नया टैब खोलता है। इस फ्लैग को true पर सेट करने से यह व्यवहार अक्षम हो जाता है:

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'इमेज रीडायरेक्ट अक्षम करें'; code-example-end]

यदि आप इमेज क्लिक को स्वयं कैप्चर करने की योजना नहीं बना रहे हैं (देखें [onImageClicked](#callbacks)), तो हम अनुशंसा करते हैं कि इसे कुछ स्टाइलिंग के साथ
जोड़ा जाए ताकि यह दिखावा हट जाए कि इमेज पर क्लिक किया जा सकता है।