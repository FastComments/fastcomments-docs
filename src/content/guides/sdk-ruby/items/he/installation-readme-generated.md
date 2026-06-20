הוסף שורה זו ל-Gemfile של היישום שלך:

```ruby
gem 'fastcomments'
```

ואז הריצו:

```bash
bundle install
```

או התקינו אותו באופן ידני כך:

```bash
gem install fastcomments
```

### תכולת הספרייה

ספרייה זו מכילה את לקוח ה-API שנוצר ואת כלי SSO כדי להקל על העבודה עם ה-API.

- [תיעוד ספריית לקוח ה-API](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### API ציבוריים לעומת API מאובטחים

ללקוח ה-API יש שלוש מחלקות, `DefaultApi`, `PublicApi` ו-`ModerationApi`. ה-`DefaultApi` מכילה שיטות שדורשות את מפתח ה-API שלך, ו-`PublicApi` מכילה קריאות API שניתן לבצע ישירות מתוך דפדפן/מכשיר נייד/וכו' ללא אימות. ה-`ModerationApi` מכילה את השיטות שמפעילות את לוח המחוונים של הממונים.

ה-`ModerationApi` מכסה פיקוח על תגובות (list, count, search, logs, export), פעולות פיקוח (remove/restore, flag, set review/spam/approval status, votes, reopen/close thread), חרמות (ban from a comment, undo, pre-ban summaries, ban status/preferences, banned-user counts), ותארים ואמון (award/remove badge, manual badges, get/set trust factor, user internal profile). כל שיטה ב-`ModerationApi` מקבלת פרמטר `sso` כך שהבקשה יכולה להתבצע בשם ממונה שאומת באמצעות SSO.