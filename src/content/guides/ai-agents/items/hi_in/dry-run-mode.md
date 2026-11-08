**Dry Run** वह सुरक्षा मोड है जिसमें हर नया एजेंट शुरू होता है। एजेंट पूरी तरह end-to-end चलता है, सिवाय उस हिस्से के जहाँ यह आपके community को छूता है।

### Dry Run में क्या चलता है

- Triggers सामान्य रूप से फायर होते हैं।
- एजेंट का prompt, [community guidelines](#community-guidelines), और [context](#context-options) असेम्बल किए जाते हैं।
- LLM को कॉल किया जाता है।
- मॉडल tool calls चुनता है और justifications + confidence scores प्रदान करता है।
- रन **Dry Run** बैज के साथ रिकॉर्ड किया जाता है ताकि यह live रन से स्पष्ट रूप से अलग दिखे।

### Dry Run में क्या नहीं चलता

- कोई कमेंट पोस्ट नहीं होता, कोई वोट नहीं डाला जाता, कोई कमेंट pin/unpin/lock/unlock नहीं होता।
- कोई कमेंट spam मार्क नहीं होता, approve नहीं होता, या review नहीं होता।
- किसी यूज़र को ban, warn, या badge नहीं दिया जाता।
- कोई ईमेल नहीं भेजा जाता।
- कोई memory नहीं लिखी जाती। (हाँ - memory भी शामिल है। Dry-run एजेंट काल्पनिक निर्णयों से shared memory pool को दूषित नहीं कर सकते।)
- Tool actions के लिए कोई webhook फायर नहीं होता। (Trigger-level `trigger.succeeded` / `trigger.failed` webhooks अभी भी फायर होते हैं और payload में `wasDryRun: true` शामिल होता है। देखें [Webhook Payloads](#webhook-payloads)।)

### इसकी लागत क्या है

Dry Run **वही LLM कॉल** करता है जो एक Enabled रन करता। Tokens चार्ज होते हैं, [budget caps](#budgets-overview) लागू होते हैं, और रन per-agent तथा per-tenant दैनिक/मासिक सीमाओं में गिने जाते हैं।

यह लागत एक वफ़ादार प्रीव्यू पाने की कीमत है। "LLM कॉल छोड़ दें" मोड आपको एजेंट के व्यवहार के बारे में कोई संकेत नहीं देगा।

### Dry-run परिणाम पढ़ना

[Run History](#run-history) में, dry-run रन status कॉलम में **Dry Run** बैज के साथ चिह्नित होते हैं। प्रत्येक रन के अंदर की actions live actions जैसी दिखती हैं - वही tool नाम, वही arguments, वही justification और confidence - सिवाय इसके कि उनमें से कोई वास्तव में नहीं हुआ।

[Analytics page](#analytics-page) प्रति माह "dry-run vs live" रन का विश्लेषण दिखाता है ताकि आप देख सकें कि आपका token खर्च कितना observation में गया।

### Dry Run से बाहर स्विच करना

एजेंट को एडिट करें और **Status** को **Enabled** में बदलें। अगला trigger live चलेगा।

आप उल्टा भी स्विच कर सकते हैं - Enabled से वापस Dry Run - यदि एजेंट ऐसे काम करने लगे जो आपको पसंद नहीं हैं। इसका कोई दंड नहीं है।

### Replays हमेशा Dry Run लागू करते हैं

[Test Runs (Replays)](#test-runs-replays) फीचर एजेंट को ऐतिहासिक कमेंट्स पर **हमेशा dry-run में** चलाता है, एजेंट की सेव की गई status की परवाह किए बिना। Replays पुराने कमेंट्स पर वास्तविक actions नहीं ले सकते। यह जानबूझकर है - replay एक प्रीव्यू टूल है, moderation टूल नहीं।