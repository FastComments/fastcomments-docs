EU क्षेत्र में रहने वाले tenants के लिए FastComments EU Digital Services Act का Article 17 लागू करता है: **पूरी तरह स्वचालित यूज़र सस्पेंशन की अनुमति नहीं है**।

### व्यवहार में इसका क्या अर्थ है

जब आपका tenant EU क्षेत्र में हो, तो agent edit फ़ॉर्म पर:

- `ban_user` के लिए **Approvals** चेकबॉक्स **locked on** रहता है और उसे untick नहीं किया जा सकता।
- लेबल पर लिखा होता है: "EU DSA Article 17: user suspensions require human review. 'Ban a user' is locked on and cannot be fully automated in the EU region."
- approval कॉलम पर tooltip में लिखा होता है: "Locked on by EU DSA Article 17 - fully-automated bans are not permitted in the EU region."

आप जो भी कॉन्फ़िगर करें, EU-क्षेत्र tenant पर किसी भी agent से आने वाली हर `ban_user` कॉल [approvals inbox](#approval-workflow) में मानव समीक्षा के लिए जाती है। मनुष्य द्वारा अनुमोदन तक ban लागू नहीं होता।

### यह प्रॉम्प्ट स्तर के बजाय प्लेटफ़ॉर्म स्तर पर क्यों लागू है

सिस्टम प्रॉम्प्ट को दुर्व्यवहार करने वाला पर्याप्त रूप से गलत मॉडल नज़रअंदाज़ कर सकता है या उनके आसपास जा सकता है। Article 17 का अनुपालन मॉडल के अच्छे व्यवहार पर निर्भर रहने के लिए बहुत महत्वपूर्ण है; यह एक कठोर server-side gate होना चाहिए जिसे tool dispatcher स्वयं लागू करे। और हम ठीक यही करते हैं।

### क्या approval से गुज़रता है और क्या नहीं

- **`ban_user`**: EU में हमेशा gated। इसमें शामिल है:
  - Visible bans (`shadowBan: false`)।
  - Shadow bans (`shadowBan: true`)।
  - `deleteAllUsersComments: true` के साथ bans।
  - `banIP: true` के साथ bans।
- सभी ban वेरिएंट agent के reasoning और confidence के साथ approvals inbox में पहुँचते हैं; मनुष्य उन्हें approve या reject करता है।

अन्य agent tools (`mark_comment_spam`, `warn_user`, `lock_comment`, आदि) Article 17 से **प्रभावित नहीं** होते। आप उन्हें अभी भी स्वचालित कर सकते हैं। Article 17 विशेष रूप से यूज़र सस्पेंशन के बारे में है।

### गैर-EU tenants का क्या

यह लॉक EU क्षेत्र के बाहर लागू नहीं होता। आप चाहें तो `ban_user` को approval के पीछे gate कर सकते हैं - किसी भी moderation agent के जीवन के पहले कुछ हफ़्तों के लिए हम इसकी दृढ़ता से सिफ़ारिश करते हैं - लेकिन यह लागू नहीं किया जाता।

### Shadow bans

Article 17 के उद्देश्यों के लिए shadow bans सस्पेंशन माने जाते हैं (यूज़र पोस्ट कर सकता है लेकिन उसकी सामग्री छिपी रहती है)। इन्हें visible bans की तरह ही gate किया जाता है।

### क्षेत्र का पता लगाना

क्षेत्र (region) प्रक्रिया स्तर पर FastComments deployment पर `REGION` environment variable द्वारा निर्धारित होता है (`models/constants.ts` में `isEURegion()` द्वारा पढ़ा जाता है)। कोई per-tenant region फ़ील्ड नहीं है - यह लॉक EU-deployed instance पर हर tenant पर लागू होता है। यदि आप अपना डेटा गैर-EU deployment से EU deployment में migrate करते हैं, तो यह लॉक उस instance पर सभी tenants के लिए प्रभावी हो जाता है।

### यदि सभी reviewers अनुपलब्ध हों

निर्णय होने तक approval inbox में पड़ा रहेगा। यह बनने के 90 दिन बाद स्वतः समाप्त (auto-expire) हो जाता है। "कोई reviewer उपलब्ध नहीं, स्वचालित निर्णय पर जाएँ" जैसा कोई रास्ता नहीं है - इससे Article 17 का उद्देश्य ही समाप्त हो जाएगा।

यदि आपका समुदाय इतना high-volume है कि EU bans की समीक्षा उचित समय में नहीं हो सकती, तो विचार करें:

- अधिक reviewers जोड़ना ([Approval Notifications](#approval-notifications) देखें)।
- Agent को [`warn_user`](#tool-warn-user) अधिक सक्रिय रूप से उपयोग करने के लिए बदलना, क्योंकि warnings Article 17 के अधीन नहीं हैं।
- [community guidelines](#community-guidelines) या [initial prompt](#personality-prompt) को कड़ा करके agent की banning की प्रवृत्ति कम करना।

### यह भी देखें

- [Tool: ban_user](#tool-ban-user) - `ban_user` क्या करता है और अतिरिक्त opt-ins के पीछे के destructive options के लिए।
- [Approval Workflow](#approval-workflow) - पूरी approval lifecycle के लिए।