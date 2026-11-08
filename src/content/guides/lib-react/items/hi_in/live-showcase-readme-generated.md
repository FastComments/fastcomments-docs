सार्वजनिक `demo` टेनेंट के विरुद्ध हर विजेट और फ़्लो को स्थानीय रूप से चलते देखने के लिए, रिपॉज़िटरी को क्लोन करें और चलाएँ:

```bash
npm install
npm run build
cd examples/example-showcase
npm install
npm run dev
```

उदाहरण लाइब्रेरी से `file:../..` के माध्यम से लिंक होते हैं, इसलिए `dist/` बनाने के लिए रूट `npm run build` चरण एक बार चलाना आवश्यक है।

प्रत्येक विजेट/फ़्लो का अपना व्यू `examples/example-showcase/src/views/` के अंतर्गत होता है, जिसे आप सीधे अपने React ऐप में कॉपी कर सकते हैं।