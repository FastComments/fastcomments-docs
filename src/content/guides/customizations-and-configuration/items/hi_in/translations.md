[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

FastComments के साथ, कमेंट विजेट का सारा टेक्स्ट कस्टमाइज़ किया जा सकता है।

आप एक टेक्स्ट को ओवरराइड कर सकते हैं, जैसे सबमिट बटन, या पूरे कमेंट विजेट का सारा टेक्स्ट।

डिफ़ॉल्ट रूप से, कमेंट विजेट का टेक्स्ट यूज़र के लोकेल के आधार पर अनुवादित होता है। हालाँकि, यदि हमें विश्वास है कि हमारे यूज़र-बेस एक ही लोकेल/भाषा का उपयोग कर रहे हैं, तो हम टेक्स्ट को ओवरराइड कर सकते हैं, उदाहरण के लिए:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'कस्टम टेक्स्ट'; code-example-end]

सभी कस्टमाइज़ेबल ट्रांसलेशन <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">यहाँ</a> "advanced options" टैब के अंतर्गत मिल सकते हैं।

हालाँकि, विजेट कस्टमाइज़ेशन UI के माध्यम से इसका एक आसान तरीका भी है। वहाँ हम EN_US लोकेल में कमेंटिंग विजेट में दिखने वाला टेक्स्ट ढूँढकर एक प्रतिस्थापन निर्दिष्ट कर सकते हैं।

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; alt='ड्रॉपडाउन से विजेट स्ट्रिंग चुने जाने और प्रतिस्थापन टेक्स्ट फ़ील्ड के साथ कस्टम टेक्स्ट पैनल'; title='कस्टम टेक्स्ट' app-screenshot-end]

अभी सभी ट्रांसलेशन ओवरराइड सभी लोकेल को प्रभावित करते हैं।

यह विजेट के अपने टेक्स्ट को कवर करता है। आपके विज़िटर द्वारा लिखे गए कमेंट्स का अनुवाद करने के लिए, [Comment Translation](#comment-translation) देखें।