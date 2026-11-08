[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

डिफ़ॉल्ट रूप से, FastComments उन टिप्पणियों के लिए "Unverified Comment" लेबल दिखाएगा जो ऐसे उपयोगकर्ता के लिए छोड़ी गई हैं जिनका ब्राउज़र सत्र सत्यापित नहीं है। असत्यापित टिप्पणी के बारे में अधिक पढ़ें [यहाँ](https://docs.fastcomments.com/guide-comment-vote-verification.html)।

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'असत्यापित लेबल अक्षम करें'; code-example-end]

इसके अतिरिक्त, इस सुविधा का उपयोग कोड लिखे बिना, कस्टमाइज़ेशन UI में किया जा सकता है:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; alt='विजेट कस्टमाइज़ेशन पेज जिसमें Disable Unverified Comment Label चेकबॉक्स चेक किया गया है'; title='असत्यापित लेबल अक्षम करें' app-screenshot-end]