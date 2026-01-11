### PyPI

```bash
pip install fastcomments
```

### תכולת הספרייה

ספרייה זו מכילה שני מודולים: לקוח ה-API שנוצר והספרייה המרכזית של Python שמכילה כלי עזר שנכתבו ידנית כדי להקל על העבודה עם ה-API, כולל תמיכה ב-SSO.

- [תיעוד ספריית לקוח ה-API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [תיעוד ספריית הליבה, כולל דוגמאות ל-SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### ממשקי API ציבוריים לעומת מאובטחים

בעבור לקוח ה-API, יש שתי מחלקות, `DefaultApi` ו-`PublicApi`. ה-`DefaultApi` מכילה שיטות שדורשות את מפתח ה-API שלך, ו-`PublicApi` מכילה קריאות API שניתן לבצע ישירות מדפדפן/מכשיר נייד/וכו' ללא אימות.