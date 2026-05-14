#### "אסימון הרישום לא נמצא, פג תוקפו, או כבר בשימוש"

האסימון בכתובת ה-URL של הרישום שלך (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">השג אותו כאן</a>) תקף למשך 30 דקות וניתן להשתמש בו רק פעם אחת. אם ה-LMS שלך לקח יותר זמן מזה, או אם ניסו לבצע רישום חוזר לאחר שהרישום כבר הצליח, האסימון יידחה. צור כתובת URL חדשה בדף התצורה של FastComments LTI 1.3 והתחל מחדש.

#### "הפלטפורמה דחתה את הרישום"

ה-LMS שלך דחה את ידית הרישום. הסיבות השכיחות ביותר:

- **Tool already registered with the same client name.** פלטפורמות מסוימות (במיוחד D2L) דוחות רישום שני של "FastComments" עד שהקודם נמחק. הסר את הכלי הישן ב-LMS שלך, ואז נסה שוב.
- **Wrong field in the LMS.** וודא שהדבקת את כתובת ה-URL לשדה **registration / tool initiation registration endpoint**, ולא לשדה launch URL או login URL.
- **The LMS doesn't actually support Dynamic Registration.** גרסאות ישנות של Moodle ו-Blackboard מצהירות תמיכה ב-LTI 1.3 אך מאפשרות רק תצורה ידנית. בדוק את התיעוד של הפלטפורמה שלך.

#### "Failed to fetch platform configuration"

FastComments לא הצליחה לקרוא את מסמך openid-configuration של ה-LMS שלך. זה נדיר ובדרך כלל אומר שה-LMS סיפק URL גילוי שבור או שאינו נגיש. פנה לתמיכת ה-LMS שלך.

#### Launch shows "Configuration not found"

או שהתצורה ב-FastComments נמחקה, או שההפעלה הגיעה מזוג `iss`/`client_id` שאיננו מכירים. אם מחקת והרשמת מחדש, הנחה את ה-LMS להסיר ולהוסיף מחדש את הכלי FastComments כדי שיקבל את ה-client_id החדש.

#### Launch shows "Deployment not registered"

הפעלת את FastComments מתוך פריסת Brightspace/Moodle/Blackboard שונה מזו שבה הוא הושק לראשונה. FastComments מקבע את ה-`deployment_id` בהשקה הראשונה כבדיקת אבטחה. כדי להוסיף פריסה חדשה תחת אותו לקוח, פנה לתמיכה — נוסיף את ה-deployment ID לתצורה.

#### Launch shows "Unsupported message_type"

ה-LMS שלח הודעת LTI ש-FastComments לא מטפל בה (למשל `LtiSubmissionReviewRequest`). FastComments תומך רק בזרימות ההשקה הסטנדרטיות resource-link וב-deep-linking. פנה אלינו אם אתה צריך שתתוסף סוג הודעה ספציפי.

#### Iframe doesn't resize

רוב ה-LMS מותאמים אוטומטית לגודל של iframes של LTI. אם ה-LMS שלך לא עושה זאת, בדוק שהגדרות ההשקה של ה-LMS מאפשרות לכלי לשלוח אירועי postMessage אל המסגרת האב. FastComments משדר גם הודעות שינוי גודל בסגנון Canvas (`lti.frameResize`) וגם לפי תקן IMS (`org.imsglobal.lti.frameResize`).