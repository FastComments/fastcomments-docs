הוסף שורה זו לקובץ Gemfile של היישום שלך:

```ruby
gem 'fastcomments'
```

ואז הפעל:

```bash
bundle install
```

או התקן זאת בעצמך כ:

```bash
gem install fastcomments
```

### Library Contents

ספרייה זו מכילה את לקוח ה‑API שנוצר ואת כלי ה‑SSO שמקלים על העבודה עם ה‑API.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Public vs Secured APIs

ללקוח ה‑API יש שלוש מחלקות: `DefaultApi`, `PublicApi` ו‑`ModerationApi`. ה‑`DefaultApi` מכילה שיטות הדורשות מפתח API שלך, וה‑`PublicApi` מכילה קריאות API שניתן לבצע ישירות מדפדפן/מכשיר נייד/וכו׳ ללא אימות. ה‑`ModerationApi` מכילה את השיטות שמפעילות את לוח המחוונים של המודרטור.

ה‑`ModerationApi` מספקת חבילה נרחבת של API מודרציה בזמן אמת ומהירה. כל שיטה ב‑`ModerationApi` מקבלת פרמטר `sso` ויכולה לאמת באמצעות SSO או קוביית סשן של FastComments.com.