Hugo थीम कंपोनेंट जोड़ने के दो मानक तरीकों में से एक चुनें।

### विकल्प A: Hugo Module (अनुशंसित)

अपनी साइट के रूट से:

```bash
hugo mod init github.com/you/your-site   # केवल तब जब आपकी साइट पहले से मॉड्यूल न हो
hugo mod get github.com/FastComments/fastcomments-hugo
```

फिर अपने `hugo.toml` में इंपोर्ट जोड़ें:

```toml
[module]
  [[module.imports]]
    path = "github.com/FastComments/fastcomments-hugo"
```

### विकल्प B: थीम कंपोनेंट (Git submodule)

```bash
git submodule add https://github.com/FastComments/fastcomments-hugo.git themes/fastcomments-hugo
```

फिर इसे अपने `hugo.toml` से रेफरेंस करें। इसे अपनी थीम के साथ सूचीबद्ध करें; बाद की प्रविष्टियाँ प्राथमिकता लेती हैं, इसलिए अपनी थीम को पहले रखें:

```toml
theme = ["your-theme", "fastcomments-hugo"]
```