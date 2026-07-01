### פעולות זמינות לכל המשתמשים

- **Flag/Unflag** -- דווח על תגובה לבדיקה

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- הסתר את כל התגובות ממשתמש (לכל צופה)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### פעולות למנהלים בלבד

- **Pin/Unpin** -- הצמד תגובה לראש השרשרת

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Lock/Unlock** -- מנע תגובות חדשות לתגובה, וחסום עריכות ומחיקות עד שנפתחת (חל על כולם, כולל ממונים)

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

כל פעולות המידור זמינות גם דרך תפריט ההקשר של התגובה בממשק המשתמש. פעולות מנהל מופיעות רק כאשר המשתמש הנוכחי הוא מנהל אתר (הוגדר באמצעות דגל `isAdmin` ב‑SSO או תצורת לוח הבקרה).