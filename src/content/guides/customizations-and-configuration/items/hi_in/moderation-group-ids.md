[related-parameter-start name = 'moderationGroupIds'; type = 'Array<string>'; related-parameter-end]

[Moderation Groups](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) पेज से जेनरेट की गई ids की एक सूची।

जब निर्दिष्ट किया जाता है, तो निर्दिष्ट कॉन्फ़िगरेशन का उपयोग करके छोड़ी गई टिप्पणियों में उसी सेट के `moderationGroupIds` होंगे।

यदि एक `Moderator` के लिए एक या अधिक [Moderation Groups](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) परिभाषित हैं, तो वे
केवल अपने समूह(s) से संबंधित `Moderate Comments` पेज की टिप्पणियाँ देखेंगे।

[code-example-start config = {moderationGroupIds: ['mxZAhjzdb', 'FT19nXbqA']}; linesToHighlight = [6, 7, 8, 9]; title = 'मॉडरेशन समूह निर्दिष्ट करें'; code-example-end]