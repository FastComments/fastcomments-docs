כל ווידג'ט מ־`fastcomments-react` זמין תחת השמות הבאים:

| Component | Handle type | Embed loaded |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | וידג'ט תגובות חיות ראשי |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | תג ספירת תגובות פנימי |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | וידג'ט צ'אט חי זורם |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | צ'אט שיתופי מעוגן לטקסט |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | תגובות תמונה מבוססות אזורים |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | פיד תגובות אחרונות |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | פיד דיונים אחרונים |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | סיכום דירוג בכוכבים |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | טבלת מובילים של דפים עם הכי הרבה תגובות |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | ציר זמן פעילות לכל משתמש |

### ווידג'טים שמתחברים לאלמנט קיים

`FastCommentsCollabChatWidget` ו־`FastCommentsImageChatWidget` מוצמדים אל אלמנט שמסופק על ידי הקורא. העבר פונקציית גישה (`targetRef`) שמחזירה את האלמנט לאחר שהוא מותקן:

```tsx
import { FastCommentsImageChatWidget } from 'fastcomments-solidjs';

export default function ImageChat() {
  let img: HTMLImageElement | undefined;
  return (
    <>
      <img ref={img} src="/screenshot.png" alt="" />
      <FastCommentsImageChatWidget
        tenantId="demo"
        urlId="my-image"
        targetRef={() => img}
      />
    </>
  );
}
```

### אזורים

העבר `region="eu"` כדי לנתב את התנועה של הווידג'ט דרך אשכול ה‑EU.