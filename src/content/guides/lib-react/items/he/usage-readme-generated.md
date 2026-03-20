### רכיב הווידג'ט הראשי

הרכיב FastCommentsCommentWidget מכיל את ווידג'ט התגובות החי של FastComments.

החליפו את "demo" למטה ב־"tenantId" שלכם - זמין [כאן](https://fastcomments.com/auth/my-account/api) באזור הניהול של FastComments.

הווידג'ט תומך בהרבה אפשרויות - ראו FastCommentsCommentWidgetConfig ב־src/index.tsx.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### עדכון הדף הנוכחי (ליישומי SPA)
כדי לעדכן את הדף/המאמר שאליו קשור שרשור התגובות יש לעדכן את פרמטרי התצורה "urlId" ו-"url".
ראו את הדוגמה וההסבר [כאן](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### אזור חשבון (תשומת לב: לקוחות באיחוד האירופי)

אם אתם באיחוד האירופי, רצוי להודיע לווידג'טים של הלקוח באיזה אזור אתם נמצאים. ראו [examples/example-eu](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-eu/src/App.tsx);
אחרת, לא חייבים להגדיר את `region`.

### ווידג'ט ספירת ההערות

הרכיב FastCommentsCommentCountWidget מכיל את ווידג'ט ספירת התגובות החי של FastComments.

החליפו את "demo" למטה ב־"tenantId" שלכם - זמין [כאן](https://fastcomments.com/auth/my-account/api) באזור הניהול של FastComments.

ראו את FastCommentsCommentCountConfig ב־src/index.tsx לאפשרויות התצורה הנתמכות.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentCountWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentCountWidget tenantId="demo" urlId="https://example.com/some-page-or-id" />
  }
}
```

### גרסה מקומית (Native)

למימוש מקומי מלא של FastComments, ראו [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

ל־wrapper של React Native עבור ספרייה זו, המשתמש ב‑webview, ראו [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).