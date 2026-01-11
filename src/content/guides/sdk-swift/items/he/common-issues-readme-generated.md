### שגיאות 401 — לא מורשה

If you're getting 401 errors when using the authenticated API:

1. **בדוק את מפתח ה-API שלך**: ודא שאתה משתמש במפתח ה-API הנכון מתוך לוח הבקרה של FastComments
2. **אמת את tenant ID**: ודא ש-tenant ID תואם לחשבונך
3. **פורמט מפתח ה-API**: מפתח ה-API צריך להיות מוגדר על ה-API client:

```swift
let defaultApi = DefaultAPI()
defaultApi.apiKey = "YOUR_API_KEY"
```

4. **שימוש ב-API לא נכון**: ודא שאתה משתמש ב-`DefaultAPI` (לא ב-`PublicAPI`) לקריאות מאומתות

### בעיות בטוקני SSO

If SSO tokens aren't working:

1. **השתמש במצב מאובטח בסביבת הייצור**: תמיד השתמש ב-`FastCommentsSSO.createSecure()` עם מפתח ה-API שלך בייצור
2. **מנוהל בצד השרת בלבד**: צור טוקנים מאובטחים של SSO על השרת שלך, לעולם אל תחשוף את מפתח ה-API ללקוחות
3. **בדוק את נתוני המשתמש**: ודא שכל השדות הנדרשים (id, email, username) מסופקים
4. **תפוגת הטוקן**: טוקני SSO מאובטחים כוללים חותם זמן ועשויים לפוג. צור טוקנים חדשים לפי הצורך.

### שגיאות SSL/TLS

If you encounter SSL/TLS errors:

1. ודא שקובץ Info.plist של האפליקציה שלך מאפשר חיבורים HTTPS לfastcomments.com
2. בדוק שאתה לא משתמש ביוצאי דופן של App Transport Security שעלולים לחסום את החיבור