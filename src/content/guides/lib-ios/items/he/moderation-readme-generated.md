### פעולות זמינות לכל המשתמשים

- **סמן/בטל סימון** -- לדווח על תגובה לבחינה

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **חסום/בטל חסימה** -- להסתיר את כל התגובות של משתמש (לצופה ספציפי)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### פעולות למנהלים בלבד

- **הצמד/בטל הצמדה** -- להצמיד תגובה לראש השרשור

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **נעל/בטל נעילה** -- למנוע תגובות חדשות על תגובה

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

כל פעולות המודרציה זמינות גם דרך תפריט ההקשר של תגובה בממשק המשתמש. פעולות מנהל מופיעות רק כאשר המשתמש הנוכחי הוא מנהל האתר (נקבע באמצעות דגל SSO `isAdmin` או בהגדרות לוח הבקרה).

---
---