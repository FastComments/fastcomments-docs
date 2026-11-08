[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

डिफ़ॉल्ट रूप से, FastComments में लाइव कमेंटिंग सक्षम रहती है।

इसका मतलब है कि कमेंट थ्रेड के हर दर्शक को एक ही कंटेंट दिखना चाहिए।

उदाहरण के लिए, यदि एक कमेंट जोड़ा जाता है, तो वह कमेंट दिखना चाहिए। यदि कोई कमेंट संपादित या हटाया जाता है,
तो वे कमेंट थ्रेड के सभी दर्शकों के लिए संपादित या हटाए जाएंगे। वोट्स और सभी मॉडरेशन कार्यों के साथ भी यही लागू होता है।

हालाँकि, हम इसे अक्षम कर सकते हैं:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'लाइव कमेंटिंग अक्षम करें'; code-example-end]

यह कोड के बिना भी किया जा सकता है। विजेट कस्टमाइज़ेशन पेज में, "Disable Live Commenting" सेक्शन देखें।

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; alt='विजेट कस्टमाइज़ेशन पेज का Disable Live Commenting सेक्शन, रीयल-टाइम थ्रेड अपडेट बंद करते हुए'; title='Disable Live Commenting' app-screenshot-end]