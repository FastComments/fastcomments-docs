#### "אסימון ההרשמה לא נמצא, פג תוקפו, או כבר בשימוש"

האסימון ב-URL ההרשמה תקף למשך 30 דקות וניתן לשימוש פעם אחת בלבד. אם ה-LMS שלך לקח יותר זמן מזה, או אם ההרשמה נותרצה אחרי שהצליחה, האסימון יידחה. צור URL חדש בדף התצורה של FastComments LTI 1.3 והתחל מחדש.

#### "הפלטפורמה דחתה את ההרשמה"

ה-LMS שלך דחה את הלחיצת היד להרשמה. הסיבות השכיחות ביותר:

- **כלי כבר רשום עם אותו שם לקוח.** פלטפורמות מסוימות (במיוחד D2L) דוחות רישום שני של "FastComments" עד שהקודם נמחק. הסר את הכלי הישן ב-LMS שלך, ואז נסה שוב.
- **שדה שגוי ב-LMS.** וודא שהדבקת את ה-URL בשדה **registration / tool initiation registration endpoint**, לא בשדה ה-launch URL או ה-login URL.
- **ה-LMS למעשה לא תומך ב-Dynamic Registration.** גרסאות ישנות של Moodle ו-Blackboard מפרסמות LTI 1.3 אך מאפשרות רק קונפיגורציה ידנית. בדוק את התיעוד של הפלטפורמה שלך.

#### "Failed to fetch platform configuration"

FastComments לא יכלה לקרוא את מסמך openid-configuration של ה-LMS שלך. זה נדיר ובדרך כלל משמעותו שה-LMS סיפק discovery URL שגוי או שאינו נגיש. פנה לתמיכת ה-LMS שלך.

#### ההשקה מציגה "התצורה לא נמצאה"

או שהתצורה ב-FastComments נמחקה, או שההשקה הגיעה מ-`iss`/`client_id` שאנחנו לא מזהים. אם מחקת ורשמת מחדש, הורת ל-LMS שלך להסיר ולהוסיף שוב את כלי FastComments כדי שיקבל את ה-client_id החדש.

#### ההשקה מציגה "הפריסה לא רשומה"

הפעלת את FastComments מתוך פריסת Brightspace/Moodle/Blackboard שונה מזו שבה בוצעה ההשקה הראשונה. FastComments מקבעת את ה-`deployment_id` בהשקה הראשונה כבדיקת אבטחה. כדי להוסיף פריסה חדשה תחת אותו client, פנה לתמיכה - נוסיף את ה-deployment ID לתצורה.

#### ההשקה מציגה "message_type לא נתמך"

ה-LMS שלח הודעת LTI ש-FastComments לא מטפל בה (למשל `LtiSubmissionReviewRequest`). FastComments תומכת רק ב-standard resource-link launch ו-deep-linking flows. פנו אלינו אם אתם זקוקים להוספת סוג הודעה ספציפי.

#### ה-iframe לא משנה גודל

ברוב מערכות ה-LMS גודל ה-iframes של LTI מותאם אוטומטית. אם אצלך זה לא קורה, בדוק שההגדרות להשקה ב-LMS מאפשרות לכלי לשלוח אירועי postMessage למסגרת האב. FastComments משדר גם הודעות שינוי גודל בסגנון Canvas (`lti.frameResize`) וגם לפי IMS-spec (`org.imsglobal.lti.frameResize`).