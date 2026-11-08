[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

जब कोई उपयोगकर्ता पहली बार FastComments के साथ कमेंट करता है, तो हम <a href="https://gravatar.com/" target="_blank">https://gravatar.com/</a> से उनका अवतार प्राप्त करने का प्रयास करेंगे।

हालाँकि, यदि हमें कोई अवतार नहीं मिलता, या उपयोगकर्ता ने अपने खाते में कभी कोई अवतार सेट नहीं किया है, तो हम एक स्थिर डिफ़ॉल्ट अवतार छवि प्रदर्शित करते हैं।

अपनी स्वयं की स्थिर अवतार छवि निर्दिष्ट करने के लिए हम *defaultAvatarSrc* सेटिंग का उपयोग कर सकते हैं।

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'डिफ़ॉल्ट अवतार को ओवरराइड करें'; code-example-end]

यह कोड के बिना भी किया जा सकता है। विजेट कस्टमाइज़ेशन पेज में, "Default Avatar" सेक्शन देखें।

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; alt='विजेट कस्टमाइज़ेशन पेज का Default Avatar सेक्शन, जहाँ आप फ़ॉलबैक अवतार छवि का URL सेट करते हैं'; title='डिफ़ॉल्ट अवतार को कस्टमाइज़ करना' app-screenshot-end]

ध्यान दें कि किसी विशेष उपयोगकर्ता के लिए अवतार परिभाषित करना, जैसे कि SSO के साथ, अपने स्वयं के सेक्शन में कवर किया गया है।