### שגיאות 401 (Unauthorized)

אם אתם מקבלים שגיאות 401 בעת שימוש ב-API מאומת:

1. **בדקו את מפתח ה-API שלכם**: ודאו שאתם משתמשים במפתח ה-API הנכון מלוח הבקרה של FastComments
2. **וודאו את ה-tenant ID**: ודאו שה-tenant ID תואם לחשבונכם
3. **פורמט מפתח ה-API**: יש להעביר את מפתח ה-API ב-Configuration:

```rust
let mut config = Configuration::new();
config.api_key = Some(ApiKey {
    prefix: None,
    key: "YOUR_API_KEY".to_string(),
});
```

### בעיות באסימוני SSO

אם אסימוני SSO לא פועלים:

1. **השתמשו במצב מאובטח עבור הייצור**: תמיד השתמשו ב-`FastCommentsSSO::new_secure()` עם מפתח ה-API שלכם בסביבת הייצור
2. **מהצד השרת בלבד**: צרו אסימוני SSO בשרת שלכם, מעולם אל תחשפו את מפתח ה-API ללקוחות
3. **בדקו את נתוני המשתמש**: ודאו שכל השדות הנדרשים (id, email, username) מסופקים

### שגיאות של runtime אסינכרוני

ה-SDK משתמש ב-tokio עבור פעולות אסינכרוניות. ודאו ש:

1. הוספתם את tokio לתלויות שלכם:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

2. אתם משתמשים ב-runtime של tokio:
```rust
#[tokio::main]
async fn main() {
    // קוד אסינכרוני שלכם כאן
}
```