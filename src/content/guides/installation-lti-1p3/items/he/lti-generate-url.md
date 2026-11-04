#### נווטו להגדרות LTI 1.3

התחברו ל-FastComments ועברו ל<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">דף הגדרות LTI 1.3 שלכם</a>.

אם לחשבון שלכם עדיין אין גישה ל-LTI, תראו את ההודעה "LTI not enabled for this account" - פנו לתמיכה כדי להפעיל אותו בתוכנית שלכם.

#### בחרו פלטפורמה (אופציונלי)

תחת **Generate a Dynamic Registration URL**, השתמשו בתפריט הנפתח **Platform** כדי לציין ל-FastComments לאיזה LMS אתם מתחברים:

- D2L Brightspace
- Moodle
- Blackboard Learn
- Sakai
- Schoology
- פלטפורמת LTI 1.3 אחרת

אתם יכולים גם להשאיר אותו על **Auto-detect**. הפלטפורמה נקראת מה-openid-configuration של ה-LMS שלכם במהלך הרישום; התפריט הנפתח רק מגדיר את התווית המוצגת עבור ההגדרה שתיווצר.

#### יצירת ה-URL

לחצו על **Generate URL**. FastComments יוצר אסימון רישום חד-פעמי ומציג לכם URL שנראה כך:

`https://fastcomments.com/lti/v1p3/register/<long-token>`

העתיקו אותו. URL זה:

- הוא **חד-פעמי** - ברגע שה-LMS שלכם קורא לו בהצלחה, האסימון נצרך.
- פג תוקפו לאחר **30 דקות** אם לא נעשה בו שימוש.
- יש לשמור אותו פרטי - כל מי שמחזיק ב-URL יכול לרשום כלי מול הטננט שלכם בתוך אותן 30 דקות.

#### הגדרות קיימות

ברגע שרישום מסתיים בהצלחה, ההגדרה החדשה מופיעה בטבלת **Existing Configurations** באותו דף, עם הפלטפורמה, ה-Issuer, ה-Client ID והסטטוס שלה. ניתן למחוק הגדרות מהטבלה הזו אם תצטרכו לבטל רישום בעתיד.