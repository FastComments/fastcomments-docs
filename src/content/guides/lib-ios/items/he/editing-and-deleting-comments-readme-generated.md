### עריכה

```swift
try await sdk.editComment(commentId: commentId, newText: "Updated text")
```

השרת מייצר מחדש את ה-HTML. התגובה המקומית מתעדכנת אוטומטית.

### מחיקה

```swift
try await sdk.deleteComment(commentId: commentId)
```

מחיקת תגובה מסירה גם את הצאצאים שלה מהעץ המקומי.

שתי הפעולות זמינות דרך תפריט ההקשר של התגובה בממשק המשתמש כאשר המשתמש הנוכחי הוא מחבר התגובה (או מנהל האתר).