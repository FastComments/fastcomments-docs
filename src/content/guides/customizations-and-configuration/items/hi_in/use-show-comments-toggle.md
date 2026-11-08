[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

डिफ़ॉल्ट रूप से, FastComments कमेंट इनपुट बॉक्स और कमेंट थ्रेड को एक ही समय पर रेंडर करता है। कुछ वर्टिकल स्पेस बचाने के लिए,
यह विजेट के साथ इंटरैक्शन होने तक किसी भी अन्य आवश्यक फ़ील्ड को भी छिपा देता है।

हालाँकि, कमेंट विजेट को एक बटन के पीछे छिपाया जा सकता है, उदाहरण के लिए:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; alt='कमेंट विजेट एक बटन के पीछे संकुचित, जो रीडर के क्लिक करने तक कमेंट की संख्या दिखाता है'; title='कमेंट देखने के लिए क्लिक करें' app-screenshot-end]

बटन अलग-अलग अनुवादित टेक्स्ट का उपयोग करता है, जो इस बात पर निर्भर करता है कि कमेंट्स वर्तमान में दिखाए जा रहे हैं या नहीं। यदि कमेंट्स छिपे हुए हैं, तो यह `translations.SHOW_COMMENTS_BUTTON_TEXT` का उपयोग करता है। यदि
कमेंट्स दिखाए जा रहे हैं, तो यह `translations.HIDE_COMMENTS_BUTTON_TEXT` का उपयोग करता है। अनुवादों में टेक्स्ट `[count]` हो सकता है, जिसे
स्थानीयकृत संख्या से बदल दिया जाएगा।

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'कमेंट दिखाने या छिपाने के लिए क्लिक करें'; code-example-end]

यह `hideCommentsUnderCountTextFormat` कॉन्फ़िगरेशन को बदलने के लिए डिज़ाइन किया गया है।

संख्या को कमेंट थ्रेड के साथ लाइव अपडेट किया जाता है। यदि कोई कमेंट नहीं हैं तो बटन नहीं दिखाया जाता है।

इसे कोड के बिना एक कस्टमाइज़ेशन रूल बनाकर और "Click to Show Comments" सक्षम करके चालू किया जा सकता है:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; alt='विजेट कस्टमाइज़ेशन पेज पर एक कस्टमाइज़ेशन रूल में Click to show comments चेकबॉक्स टिक किया हुआ'; title='Click to Show Comments सक्षम करें' app-screenshot-end]