ארבעה vals ציבוריים שאתה יכול לשנות, כל אחד מכסה חלק אחד של המדריך הזה.

**[בלוג עם תגובות](https://www.val.town/x/fastcomments/blog-with-comments)** ([חי](https://fastcomments-blog.val.run)) הוא בלוג Markdown עם שרשור תחת כל פוסט וספירת תגובות מרוכזת באינדקס. הוא פועל ברגע שאתה משנה אותו, ומשתנה סביבתי אחד מצביע אותו לחשבון שלך.

**[הדגמת SSO](https://www.val.town/x/fastcomments/sso-demo)** ([חי](https://fastcomments-sso.val.run)) מחבר את המבקר עם חשבון Val Town שלו ומעביר את הזהות לווידג'ט, כך שאין צורך בכניסה שנייה.

**[מקבל Webhook](https://www.val.town/x/fastcomments/webhook-receiver)** ([חי](https://fastcomments-webhooks.val.run)) מאמת את חתימת ה‑HMAC בכל משלוח ושומר אירועים ב‑SQLite. יש לו כפתור שמחתום על מטען בדיקה ומספק אותו לעצמו, כך שאתה יכול לצפות באימות מוצלח לפני קביעת Webhook אמיתי.

**[כישורי סוכן](https://www.val.town/x/fastcomments/skills)** ([חי](https://fastcomments-skills.val.run)) היא ספרייה של כישורי סוכן FastComments המכסים את הווידג'ט, SSO, ה‑REST API, מודרציה והמעבר מ‑Disqus. שנה אותה והסוכן של Val Town, Townie, יטען את הכישורים מתוך `skills/` אוטומטית, כך שהסוכן שלך יודע איך לחבר תגובות ללא צורך בהדבקת תיעוד בצ'אט.

הכישורים זהים מותקנים בכל מקום אחר עם `npx skills add fastcomments/skills`.