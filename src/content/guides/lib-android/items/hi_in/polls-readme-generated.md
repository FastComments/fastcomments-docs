पोल वेब विजेट की तरह ही काम करते हैं, और आपके FastComments डैशबोर्ड में उसी विजेट सेटिंग्स से चालू होते हैं (या ऊपर दिए गए `pollCreation` / `pollVoting` से)।

- पोल वाले कमेंट्स उनके टेक्स्ट के नीचे उसे दिखाते हैं। पाठक किसी विकल्प पर टैप करके वोट करते हैं, वोट करने से पहले परिणाम दिखा या छिपा सकते हैं, और अपना वोट बदल सकते हैं। जैसे ही अन्य लोग वोट करते हैं, गिनती लाइव अपडेट होती है।
- पोल किसी तारीख पर बंद हो सकते हैं, वोट करने तक परिणाम छिपा सकते हैं, और वोटर्स को एडमिन्स या सभी को दिखा सकते हैं ("See who voted")।
- जब दर्शक पोल बना सकता हो, तो `FastCommentsView` के कमेंट इनपुट में एक पोल बटन दिखता है जो पोल बिल्डर खोलता है। इसके लिए टूलबार पंक्ति तब भी दिखती है जब आपने कभी टूलबार चालू न किया हो; `sdk.setCommentToolbarEnabled(false)` इसे छिपा देता है, पोल बटन समेत, और इनपुट पर `setPollCreationEnabled(false)` केवल पोल बटन हटाता है। `FastCommentsView` बनाने से पहले `setCommentToolbarEnabled` कॉल करें, क्योंकि इनपुट SDK प्राप्त करते समय इसे पढ़ता है; पहले से स्क्रीन पर मौजूद व्यू के लिए, इसके बजाय `getBottomCommentInput().setToolbarVisible(false)` कॉल करें। लाइव चैट इनपुट इसे छोड़ देता है, जैसे वेब लाइव चैट।

अपने अपने UI से पोल पोस्ट करने के लिए, `postComment` को एक `CommentPollInput` पास करें:

```java
CommentPollInput poll = new CommentPollInput()
        .question("Where should we eat?")
        .options(Arrays.asList("Tacos", "Ramen"))
        // वैकल्पिक: एक ISO 8601 बंद होने की तारीख, PollPrivacy मान, और वोट करने तक छिपे परिणाम।
        .closesAt(Instant.now().plus(1, ChronoUnit.DAYS).toString())
        .privacy(PollPrivacy.NUMBER_2.getValue())
        .requireVoteToSeeResults(true);
sdk.postComment("Lunch vote", null, null, poll, callback);
```

`sdk.votePoll(commentId, optionId, callback)` और `sdk.getPollVoters(commentId, page, search, callback)` वोटिंग और वोटर सूची को कवर करते हैं। पोल के रंग THEMING.md में सूचीबद्ध हैं।