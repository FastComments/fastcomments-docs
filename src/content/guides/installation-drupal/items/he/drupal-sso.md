FastComments משתלב עם מערכת המשתמשים של Drupal דרך SSO, או התחברות יחידה (single-sign-on). המשתמשים שלך נכנסים לאתר ה-Drupal שלך, והמודול מעביר את זהותם ל-FastComments אוטומטית. אין חשבונות נוספים ליצירה, ואין צורך בהרצת סנכרון ראשוני.

המודול תומך בשלוש מצבי SSO, הנקבעים תחת `Administration > Configuration > Content > FastComments`.

### ללא SSO

אין SSO. משתמשים מתפרסמים כאורחים או יוצרים חשבון FastComments. השתמש בכך אם האתר שלך ציבורי ולא צריך לקשר תגובות למשתמשי Drupal.

### פשוט

מעביר את השם, האימייל ותמונת הפרופיל של משתמש ה-Drupal ל-FastComments ללא אימות בצד השרת. אין צורך ב-API Secret. מתאים לאתרים פנימיים או בעלי סיכון נמוך.

### מאובטח (מומלץ)

משתמש ב-[HMAC-SHA256](https://en.wikipedia.org/wiki/HMAC) כדי לאמת כל זהות משתמש מול FastComments. זה המצב הרצוי כשיש לך API Secret מוגדר, וזה המצב היחיד שמונע ממבקר להתחזות למשתמש אחר.

זהות המשתמש מועברת ל-FastComments בכל פעם שמשתמש צופה בנושא תגובות. אין סנכרון ראשוני או רציף שצריך לרוץ.

<sup>(אופציונלי)</sup> הוסף את המנהלים שלך ל-[משתמשים ומנהלים](https://fastcomments.com/auth/my-account/users) ואת המנחים ל-[מנחי תגובות](https://fastcomments.com/auth/my-account/moderate-comments/moderators) כדי לשפר את חווייתם ולהפעיל מעקב סטטיסטי עבור המנחים.

למבט מעמיק יותר על איך SSO עובד, ראו את [סעיף ה‑SSO](/guide-customizations-and-configuration.html#sso) במסמכי ההתאמות האישיות.

---