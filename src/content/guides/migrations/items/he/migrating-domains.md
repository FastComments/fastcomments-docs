FastComments מספקת דרך אוטומטית עבורך להעביר את ההערות שלך בין תחומים.

הגירת התחומים דורשת פשוט תחום `from` ותחום `to`.

זה **מעביר** הערות, הוא אינו מעתיק אותן. אם ברצונך להעתיק הערות, פנה אלינו.

[app-screenshot-start url='/auth/my-account/manage-data/migrate-domains?demo=true'; linkUrl='/auth/my-account/manage-data/migrate-domains'; selector = '.content'; alt='כלי הגירת תחומים עם שדות התחום ממקור ויעד וספירת ההערות שהועברו'; title='הגירת תחומים' app-screenshot-end]

זה גם שימושי, לדוגמה, אם חלק מההגירה שלך ל‑FastComments כולל העברה מספק אחר, ולכן נתוני ייבוא ההערות שלך עשויים להכיל נתונים שצריך להעביר. במקרה זה, ניתן להריץ את הייבוא, ולאחר מכן לבצע את הגירת התחומים.

### Monitoring Progress

כלי הגירת התחומים משתמש במערכת עיבוד המשימות של FastComments זהה לזו של כלי ניהול הנתונים האחרים.

ייתכן שיהיה עיכוב לפני שההגירה שלך תתחיל. זה נורמלי, מכיוון שהמערכת בודקת משימות חדשות לעיבוד באופן תקופתי.

כאשר המשימה רצה, היא תציג את מספר ההערות שנמצאו להעברה, ואת מספר ההערות שהועברו עד כה.

### Migrating Domains via API

ניתן גם להעביר תחומים באמצעות [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post), כאשר `type` מוגדר ל‑`domain`. התקדמות זמינה מ‑[`GET /api/v1/batch-jobs/:id`](/guide-api.html#batch-job-get-by-id), וההגירה ניתנת לביטול במשך 90 יום.