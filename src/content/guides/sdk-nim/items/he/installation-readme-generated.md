### שימוש ב-Nimble

```bash
nimble install fastcomments
```

### בנייה מהמקור

```bash
nimble build
```

### תכולת הספרייה

ספרייה זו מכילה את לקוח ה-API שנוצר ואת כלי ה-SSO להקל על העבודה עם ה-API.

- [תיעוד ספריית לקוח ה-API](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### API ציבוריים לעומת API מאובטחים

ללקוח ה-API, יש שני מודולים של API, `api_default` ו-`api_public`. ה-`api_default` מכיל שיטות שדורשות את מפתח ה-API שלך, ו-`api_public` מכיל קריאות API שניתן לבצע ישירות מדפדפן/מכשיר נייד/וכו' ללא אימות.