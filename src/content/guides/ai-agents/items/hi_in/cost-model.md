एजेंट की लागत **token-आधारित** होती है। हर LLM कॉल एक token count लौटाती है, प्लेटफ़ॉर्म उसे मॉडल की per-token दर के अनुसार USD सेंट में बदलता है, और वे सेंट एजेंट तथा tenant के बजटों के विरुद्ध बिल किए जाते हैं।

### क्या बिल होता है

- **सभी LLM कॉल**, जिसमें वह कॉल भी शामिल है जो zero tool actions उत्पन्न करती है ("एजेंट ने कुछ न करने का निर्णय लिया")। कोई action न होने पर भी inference का भुगतान करना पड़ता है।
- **Dry-run कॉल**। Dry-run का मतलब है "action न लें, लेकिन LLM को कॉल करें" - LLM कॉल की लागत समान रहती है। देखें [Dry-Run Mode](#dry-run-mode)।
- **Replay कॉल**। Replays ऐतिहासिक comments के विरुद्ध dry-run runs होते हैं। इनकी लागत tokens में होती है। देखें [Test Runs (Replays)](#test-runs-replays)।

### क्या बिल नहीं होता

- **वे triggers जो कभी LLM कॉल उत्पन्न नहीं करते।** LLM से पहले drop होने वाले cases (over budget, rate limited, scope mismatch, billing invalid, loop prevention) की लागत zero tokens होती है। देखें [Drop Reasons](#drop-reasons)।
- **Tool dispatch।** `pin_comment` या किसी अन्य tool को कॉल करने पर स्वयं tokens की लागत नहीं लगती - केवल LLM round-trip की लागत लगती है।
- **`search_memory`।** यह read-only है और अपना कोई LLM round-trip उत्पन्न नहीं करता।

### प्रति run लागत

एक single agent run में LLM को कई बार कॉल किया जा सकता है - प्रत्येक tool call का result मॉडल को वापस भेजा जाता है ताकि वह या तो दूसरा tool कॉल करे या पूरा हो जाए। इसलिए किसी run पर `tokensUsed` उस run के सभी LLM round-trips का योग होता है।

प्रति-run token लागत के सबसे बड़े योगदानकर्ता:

- **लंबे [initial prompts](#personality-prompt) और [community guidelines](#community-guidelines)** - ये हर run में शामिल होते हैं।
- **[Context options](#context-options)** - thread context, user history, page metadata। प्रत्येक tokens जोड़ता है।
- **Comment text स्वयं** - लंबे comments की लागत अधिक होती है।
- **एक run में कई tool calls** - प्रत्येक tool का result message मॉडल को वापस भेजा जाता है।
- **Memory reads** - `search_memory` अधिकतम 25 records लौटाता है (कुल content 8000 chars पर capped)। उनमें से अधिकांश bytes अगले prompt में चले जाते हैं।

**Max Tokens Per Trigger** (डिफ़ॉल्ट 20,000) प्रत्येक LLM कॉल के **response** के आकार को सीमित करता है। यह input के आकार को सीमित नहीं करता।

### Token-से-सेंट रूपांतरण

प्लेटफ़ॉर्म एक single per-tenant-package दर लागू करता है (`flexLLMUnit` tokens प्रति `flexLLMCostCents`)। Cost-per-token package-level होती है, per-model नहीं - दोनों उपलब्ध मॉडल ([GLM 5.1 और GPT-OSS Turbo](#choosing-a-model)) किसी दिए गए package पर समान दर पर बिल होते हैं। [Run Detail View](#run-detail-view) run पूरा होने पर आपकी मुद्रा में प्रति-run लागत दिखाता है।

### लागत कहाँ रिकॉर्ड होती है

प्रत्येक run अपनी raw token count और प्रति-run लागत रिकॉर्ड करता है। दैनिक और मासिक totals [Analytics page](#analytics-page) में roll up होते हैं।

### लागत कैसे पढ़ें

- **प्रति-run लागत**: [Run Detail View](#run-detail-view) -> `Cost` field।
- **दैनिक / मासिक aggregate**: [Analytics page](#analytics-page) -> Budget usage और Daily cost charts।
- **प्रति-action लागत**: Run Detail View पर ही, तब उपयोगी जब किसी एजेंट का tool-loop असामान्य रूप से लंबा हो और tuning करनी हो।

### यह भी देखें

- [Choosing a Model](#choosing-a-model) - लागत पर सबसे बड़ा प्रभाव।
- [Context Options](#context-options) - बढ़ी हुई लागत कहाँ से आती है।
- [Budgets Overview](#budgets-overview) - hard caps जो बेकाबू लागत को रोकते हैं।