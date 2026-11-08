जब कोई उपयोगकर्ता एक टिप्पणी या वोट छोड़ता है, और वे लॉग इन नहीं हैं, या उनका खाता सत्यापित नहीं है, तो उन्हें इस क्रिया को सत्यापित करने के लिए कहते हुए एक ईमेल प्राप्त होगा।

हालाँकि, हम अपनी पूरी कोशिश करते हैं कि आपके उपयोगकर्ताओं को ईमेल के साथ स्पैम न करें, और प्रति सत्र एक से अधिक सत्यापन ईमेल नहीं भेजेंगे। अधिक जानकारी के लिए Sessions अनुभाग देखें।

डिफ़ॉल्ट रूप से, टिप्पणी सत्यापन ईमेल निम्नलिखित जैसे दिखते हैं:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&removedInDaysCount=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Alexander की टिप्पणी को उद्धृत करते हुए डिफ़ॉल्ट सत्यापन ईमेल का मुख्य भाग, पोस्ट की पुष्टि के लिए एक बटन के साथ'; title='टिप्पणी सत्यापन ईमेल' app-screenshot-end]

डिफ़ॉल्ट रूप से, वोट सत्यापन ईमेल निम्नलिखित जैसे दिखते हैं:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Devon से वोट की पुष्टि करने के लिए कहता हुआ डिफ़ॉल्ट ईमेल, जिसमें वोट की गई टिप्पणी और एक पुष्टि बटन दिखाया गया है'; title='वोट सत्यापन ईमेल' app-screenshot-end]

डिफ़ॉल्ट रूप से, FastComments इन ईमेल के फुटर में अपना लोगो और नाम दिखाएगा:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; alt='सत्यापन ईमेल का निचला भाग, जिसमें फुटर में डिफ़ॉल्ट FastComments लोगो और नाम दिखाया गया है'; title='ईमेल फुटर' app-screenshot-end]

यदि आप Flex या Pro टियर पर हैं, तो [The from name, email, and branding can be customized](/guide-multiple-sites.html#from-name-email-logo)।