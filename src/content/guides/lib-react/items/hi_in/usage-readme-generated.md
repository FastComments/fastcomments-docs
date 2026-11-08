### मुख्य विजेट कंपोनेंट

FastCommentsCommentWidget कंपोनेंट में लाइव FastComments कमेंट विजेट होता है।

नीचे "demo" की जगह अपना "tenantId" डालें - जो FastComments एडमिन एरिया में [यहाँ](https://fastcomments.com/auth/my-account/api) उपलब्ध है।

विजेट बहुत सारे विकल्प सपोर्ट करता है - src/index.tsx में FastCommentsCommentWidgetConfig देखें।

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### वर्तमान पेज अपडेट करना (SPAs के लिए)
कमेंट थ्रेड से जुड़े पेज/आर्टिकल को अपडेट करने के लिए आपको कॉन्फ़िगरेशन पैरामीटर "urlId" और "url" अपडेट करने होंगे।
उदाहरण और व्याख्या [यहाँ](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx) देखें।

### अकाउंट रीजन (ध्यान दें: EU ग्राहक)

यदि आप EU में हैं, तो आपको क्लाइंट विजेट्स को बताना होगा कि आप किस रीजन में हैं। [examples/example-eu](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-eu/src/App.tsx) देखें;
अन्यथा, आपको `region` परिभाषित करने की आवश्यकता नहीं है।

### कमेंट काउंट विजेट

FastCommentsCommentCountWidget कंपोनेंट में लाइव FastComments कमेंट काउंट विजेट होता है।

नीचे "demo" की जगह अपना "tenantId" डालें - जो FastComments एडमिन एरिया में [यहाँ](https://fastcomments.com/auth/my-account/api) उपलब्ध है।

सपोर्टेड कॉन्फ़िगरेशन विकल्पों के लिए src/index.tsx में FastCommentsCommentCountConfig देखें।

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentCountWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentCountWidget tenantId="demo" urlId="https://example.com/some-page-or-id" />
  }
}
```

### नेटिव

FastComments का पूरी तरह नेटिव इम्प्लीमेंटेशन देखने के लिए, [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk) देखें।

इस लाइब्रेरी का React Native रैपर (webview का उपयोग करते हुए) देखने के लिए, [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native) देखें।