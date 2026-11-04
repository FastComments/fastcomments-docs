הוסיפו את השורה הבאה ל-Gemfile של האפליקציה שלכם:

```ruby
gem 'fastcomments'
```

ולאחר מכן הריצו:

```bash
bundle install
```

או התקינו בעצמכם באמצעות:

```bash
gem install fastcomments
```

### תוכן הספרייה

ספרייה זו מכילה את לקוח ה-API שנוצר באופן אוטומטי ואת כלי ה-SSO שמקלים על העבודה עם ה-API.

- [תיעוד ספריית לקוח ה-API](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### APIs ציבוריים מול מאובטחים

עבור לקוח ה-API קיימות שלוש מחלקות: `DefaultApi`, `PublicApi`, ו-`ModerationApi`. ה-`DefaultApi` מכילה מתודות הדורשות את מפתח ה-API שלכם, וה-`PublicApi` מכילה קריאות API
שניתן לבצע ישירות מדפדפן/מכשיר נייד/וכו' ללא אימות. ה-`ModerationApi` מכילה את המתודות שמניעות את לוח הבקרה של המנחה.

ה-`ModerationApi` מספקת חבילה נרחבת של APIs לניהול מודרציה מהיר ובזמן אמת. כל מתודה של `ModerationApi` מקבלת פרמטר `sso` ויכולה לבצע אימות באמצעות SSO או עוגיית סשן של FastComments.com.