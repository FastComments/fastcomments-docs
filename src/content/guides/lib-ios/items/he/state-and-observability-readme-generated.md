---
שני `FastCommentsSDK` ו־`FastCommentsFeedSDK` הם מחלקות מסוג `ObservableObject` עם תכונות `@Published`. ניתן לצפות בהן בתוך תצוגות SwiftUI שלך לקבלת עדכוני ממשק משתמש ריאקטיביים.

### תכונות Published של FastCommentsSDK

| Property | Type | Description |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | מספר כולל של תגובות בשרת |
| `newRootCommentCount` | `Int` | תגובות חדשות שהצטברו (כאשר `showLiveRightAway` הוא false) |
| `currentUser` | `UserSessionInfo?` | המשתמש המאומת הנוכחי |
| `isSiteAdmin` | `Bool` | האם המשתמש הנוכחי הוא מנהל האתר |
| `isClosed` | `Bool` | האם שרשור התגובות סגור |
| `hasBillingIssue` | `Bool` | האם קיימת בעיית תשלום |
| `isLoading` | `Bool` | האם בקשת רשת בתהליך |
| `hasMore` | `Bool` | האם קיימים עוד דפי תגובות |
| `blockingErrorMessage` | `String?` | שגיאה החוסמת את פעולת הממשק |
| `warningMessage` | `String?` | הודעת אזהרה לא חוסמת |
| `isDemo` | `Bool` | האם פועל במצב הדגמה |
| `commentsVisible` | `Bool` | מתג להצגת תגובות |
| `toolbarEnabled` | `Bool` | האם סרגל העיצוב מוצג |

### תכונות Published של FastCommentsFeedSDK

| Property | Type | Description |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | פוסטים של ה-feed שטעונים כרגע |
| `hasMore` | `Bool` | האם קיימים עוד דפים |
| `currentUser` | `UserSessionInfo?` | המשתמש המאומת הנוכחי |
| `blockingErrorMessage` | `String?` | הודעת שגיאה חוסמת |
| `isLoading` | `Bool` | האם בקשת רשת בתהליך |
| `newPostsCount` | `Int` | מספר פוסטים חדשים מאז הטעינה האחרונה |

### עץ התגובות

ניתן לגשת לעץ התגובות דרך `sdk.commentsTree`:

```swift
// רשימה שטוחה של צמתים גלויים להצגה
sdk.commentsTree.visibleNodes

// חיפוש תגובה לפי מזהה
sdk.commentsTree.commentsById["comment-id"]
```

---
---