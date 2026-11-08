[related-parameter-start name = 'mentionGroupIds'; type = 'Array<string>'; related-parameter-end]

**@mentions** ऑटोकम्प्लीट के लिए उपयोग करने हेतु ids की एक सूची। उपयोगी जब आप यह रोकना चाहते हैं कि ऐसे उपयोगकर्ताओं को टैग किया जाए जिनके समूह आपस में मेल नहीं खाते।

जब निर्दिष्ट किया जाता है, तो `@` कैरेक्टर टाइप करने के बाद ऑटोकम्प्लीट में केवल अन्य समूहों के उपयोगकर्ता ही दिखाए जाएंगे।

[code-example-start config = {mentionGroupIds: ['yxZAhjzda', 'QT19nXbqB']}; linesToHighlight = [6, 7, 8, 9]; title = 'Mentions के लिए समूह सीमित करें'; code-example-end]