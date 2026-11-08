टैग किए गए उपयोगकर्ताओं को एक ईमेल प्राप्त होगा जिससे उन्हें पता चलेगा कि उन्हें किसी टिप्पणी में टैग किया गया है, या उल्लेख (mention) किया गया है।

[app-screenshot-start url='/test-e2e/email/comment-user-mention?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"Hey%20%40winrid%20I%20wanted%20you%20to%20see%20this."%2C"commentHTML"%3A"Hey%20<b>%40winrid<%2Fb>%20I%20wanted%20you%20to%20see%20this."%2C"date"%3A1633998787864%2C"pageTitle"%3A"Some%20Page%20Title"%7D&username=winrid&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&INTRO=Hey%20winrid%2C&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&unsubscribeLink=%7B"url"%3A"%2Fauth%2Fmy-account%2Fedit-notifications"%2C"textId"%3A"UNSUBSCRIBE_HERE"%7D&viewCommentUrl=https%3A%2F%2Fexample.com%23fast-comments-jt%3Dsome-db-id&locale=en_us&canReplyByEmail=true&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='एक टिप्पणी का उद्धरण करने वाली सूचना ईमेल का मुख्य भाग, जिसमें @winrid उल्लेख बोल्ड में है, साथ ही देखें और अनसब्सक्राइब लिंक'; title='User Mentioned Notification' app-screenshot-end]

सूचनाएं बंद करने से ये ईमेल नहीं भेजे जाएंगे, और हर ईमेल में एक हेडर प्रदान किया जाता है ताकि ईमेल क्लाइंट उपयोगकर्ता को सहजता से अनसब्सक्राइब कर सकें।

टैग किए गए उपयोगकर्ताओं को इन-ऐप सूचना भी मिलती है, तब भी जब उनका कोई ईमेल पता दर्ज न हो।

यदि टिप्पणी मॉडरेटर की स्वीकृति की प्रतीक्षा में है, तो उल्लेख ईमेल को रोक दिया जाता है और टिप्पणी के स्वीकृत होने पर भेजा जाता है। यदि टिप्पणी को स्पैम चिह्नित किया गया है, हटा दिया गया है, या 7 दिनों के भीतर स्वीकृत नहीं किया गया है, तो यह नहीं भेजा जाता।

यह देखने के लिए कि उल्लेखित उपयोगकर्ता को ईमेल भेजा गया था या नहीं, या क्यों नहीं भेजा गया, Moderate Comments पृष्ठ से उस टिप्पणी के [Comment Logs](/guide-moderation.html#comment-logs) खोलें।