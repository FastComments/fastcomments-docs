यह लाइब्रेरी [FastComments](https://fastcomments.com) का एक पूर्ण react-native कार्यान्वयन है।

यह लाइव कमेंटिंग, चैट, थ्रेड्स, इमोटिकॉन्स, नोटिफिकेशन, SSO, स्किन्स, और स्टाइलशीट ऑब्जेक्ट पास करके पूर्ण अनुकूलन का समर्थन करती है। सभी एसेट्स को भी अनुकूलित किया जा सकता है, और यह डार्क मोड के आधार पर अलग-अलग एसेट्स टॉगल करने का समर्थन करती है।

इस लाइब्रेरी का लाभ यह है कि यह `fastcomments-react-native` रैपर से अधिक लचीली है। कमेंट्स वेबव्यू के अंदर के बजाय नेटिव कंपोनेंट्स के साथ रेंडर किए जाते हैं।

यह सब FastComments बैकएंड पर चलता है, इसलिए आपको केवल UI को शामिल करना है:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

अधिक उदाहरणों के लिए [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/tree/main/example/src) देखें।

अपने मौजूदा React Native एप्लिकेशन में लाइव चैट जोड़ें, या यहां तक कि एक सोशल नेटवर्क भी बनाएं!