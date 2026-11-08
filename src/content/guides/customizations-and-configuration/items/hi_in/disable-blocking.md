[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

डिफ़ॉल्ट रूप से, FastComments उपयोगकर्ताओं को अन्य उपयोगकर्ताओं को ब्लॉक करने की अनुमति देता है। किसी उपयोगकर्ता को ब्लॉक करने से उनकी टिप्पणियाँ छिप जाती हैं, उपयोगकर्ताओं के बीच सूचनाएँ रुक जाती हैं, और इसी तरह की सुविधाएँ प्रभावित होती हैं।

हो सकता है कि आप इस सुविधा को अक्षम करना चाहें। इसे इस प्रकार किया जा सकता है:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'ब्लॉकिंग अक्षम करें'; code-example-end]

यह कोड के बिना भी किया जा सकता है, जिससे उचित सर्वर-साइड सत्यापन भी सक्षम होता है, Widget Customization UI के माध्यम से:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; alt='Widget customization UI में ब्लॉकिंग अक्षम करने का विकल्प, जो उपयोगकर्ताओं को एक-दूसरे को ब्लॉक करने से रोकता है'; title='ब्लॉकिंग अक्षम करें' app-screenshot-end]