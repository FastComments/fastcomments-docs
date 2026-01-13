---
הוסף שורה זו ל-Gemfile של היישום שלך:

```ruby
gem 'fastcomments'
```

ואז הרץ:

```bash
bundle install
```

או התקן אותו בעצמך כך:

```bash
gem install fastcomments
```

### תכולת הספרייה

ספרייה זו מכילה את לקוח ה-API שנוצר ואת כלי ה-SSO כדי להקל על העבודה עם ה-API.

- [תיעוד ספריית לקוח ה-API](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### APIs ציבוריים לעומת מאובטחים

עבור לקוח ה-API יש שתי מחלקות, `DefaultApi` ו-`PublicApi`. ה-`DefaultApi` מכילה שיטות שדורשות את מפתח ה-API שלך, ו- PublicApi מכילה קריאות API שניתן לבצע ישירות מדפדפן/מכשיר נייד/וכו' ללא אימות.
---