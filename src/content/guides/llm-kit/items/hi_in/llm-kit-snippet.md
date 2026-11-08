अपने प्रोजेक्ट की AI असिस्टेंट कॉन्फ़िगरेशन फ़ाइलों (जैसे AGENT.md या CLAUDE.md) में यह स्निपेट जोड़ें, जो AI कोडिंग असिस्टेंट्स को FastComments दस्तावेज़ीकरण खोजने और एक्सेस करना सिखाता है।

[inline-code-attrs-start title = 'FastComments LLM Kit'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
## FastComments Integration

FastComments एक एम्बेड करने योग्य लाइव कमेंटिंग प्लेटफ़ॉर्म है, जिसमें कई फ्रंटएंड और बैकएंड इंटीग्रेशन के लिए लाइब्रेरीज़, साथ ही क्लाइंट और सर्वर-साइड SDK और API उपलब्ध हैं।

FastComments के साथ काम करते समय, आप इस API को कॉल करके FastComments दस्तावेज़ीकरण खोज सकते हैं:

https://docs-search.fastcomments.com/search?query=<search_query>&full=true&tenantId=demo

`<search_query>` को अपने खोज शब्दों (URL एनकोडेड) से बदलें। API कार्यान्वयन में मदद के लिए प्रासंगिक दस्तावेज़ीकरण स्निपेट लौटाता है।
[inline-code-end]

इस तरह आपका AI असिस्टेंट आसानी से नवीनतम जानकारी प्राप्त कर सकता है और कम समय में अधिक सटीक कार्यान्वयन प्रदान कर सकता है।

### उपयोग उदाहरण

Claude Code या Cursor जैसे AI असिस्टेंट का उपयोग करते समय, आप ऐसे प्रश्न पूछ सकते हैं:

- "मैं React पर FastComments कैसे इंस्टॉल करूँ?"
- "मुझे दिखाएँ कि FastComments के साथ SSO कैसे कॉन्फ़िगर करें"
- "FastComments API प्रमाणीकरण विकल्प क्या हैं?"

AI असिस्टेंट दिए गए API एंडपॉइंट का उपयोग करके स्वचालित रूप से दस्तावेज़ीकरण खोजेगा और आधिकारिक दस्तावेज़ीकरण के आधार पर सटीक, नवीनतम उत्तर देगा।