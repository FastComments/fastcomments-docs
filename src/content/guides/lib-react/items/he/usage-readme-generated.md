### הרכיב הראשי של הווידג'ט

הקומפוננטה FastCommentsCommentWidget מכילה את ווידג'ט התגובות החי של FastComments.

החליפו את "demo" למטה ב-"tenantId" שלכם — זמין [כאן](https://fastcomments.com/auth/my-account/api) באזור הניהול של FastComments.

הווידג'ט תומך בהרבה אפשרויות - ראה FastCommentsCommentWidgetConfig ב-src/index.tsx.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### עדכון הדף הנוכחי (ל-SPAs)
כדי לעדכן את הדף/המאמר שאליו מקושר שרשור התגובות צריך לעדכן את פרמטרי התצורה "urlId" ו-"url".
ראו את הדוגמה וההסבר [כאן](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### אזור החשבון (תשומת לב: לקוחות באיחוד האירופי)

אם אתם באיחוד האירופי, רצוי להודיע לווידג'טים של הלקוח באיזה אזור אתם נמצאים. ראו [examples/example-eu](/examples/example-eu/src/App.tsx);
אחרת, אין צורך להגדיר את `region`.

### ווידג'ט ספירת התגובות

הקומפוננטה FastCommentsCommentCountWidget מכילה את ווידג'ט ספירת התגובות החי של FastComments.

החליפו את "demo" למטה ב-"tenantId" שלכם — זמין [כאן](https://fastcomments.com/auth/my-account/api) באזור הניהול של FastComments.

ראו את FastCommentsCommentCountConfig ב-src/index.tsx עבור אפשרויות התצורה הנתמכות.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentCountWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentCountWidget tenantId="demo" urlId="https://example.com/some-page-or-id" />
  }
}
```

### Native

ליישום מקומי מלא (Native) של FastComments, ראו [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

ל-wrapper ל-React Native של ספרייה זו, המשתמש ב-webview, ראו [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).