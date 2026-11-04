#### "אסימון הרישום לא נמצא, פג תוקפו, או כבר נוצל"

האסימון בכתובת הרישום שלכם (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">קבלו אותו כאן</a>) תקף למשך 30 דקות וניתן לשימוש חד-פעמי בלבד. אם ה-LMS שלכם לקח יותר זמן מכך, או אם הרישום נוסה שוב לאחר שהצליח, האסימון יידחה. צרו כתובת חדשה בעמוד הגדרות LTI 1.3 של FastComments והתחילו מחדש.

#### "הפלטפורמה דחתה את הרישום"

ה-LMS שלכם סירב לביצוע ידנית הרישום. הסיבות הנפוצות ביותר:

- **הכלי כבר רשום עם אותו שם לקוח.** חלק מהפלטפורמות (בעיקר D2L) דוחות רישום שני של "FastComments" עד שהרישום הקודם יימחק. הסירו את הכלי הישן ב-LMS שלכם ואז נסו שוב.
- **שדה שגוי ב-LMS.** ודאו שהדבקתם את הכתובת בשדה **registration / tool initiation registration endpoint**, ולא בשדה כתובת ההפעלה (launch URL) או כתובת ההתחברות (login URL).
- **ה-LMS למעשה אינו תומך ברישום דינמי (Dynamic Registration).** גרסאות ישנות יותר של Moodle ו-Blackboard מצהירות על תמיכה ב-LTI 1.3 אך מאפשרות רק הגדרה ידנית. בדקו את התיעוד של הפלטפורמה שלכם.

#### "נכשל באחזור תצורת הפלטפורמה"

FastComments לא הצליח לקרוא את מסמך ה-openid-configuration של ה-LMS שלכם. זה נדיר ובדרך כלל משמעותו שה-LMS סיפק כתובת גילוי (discovery URL) פגומה או בלתי נגישה. פנו לתמיכה של ה-LMS שלכם.

#### ההפעלה מציגה "Configuration not found"

או שהתצורה ב-FastComments נמחקה, או שההפעלה הגיעה מזוג `iss`/`client_id` שאינו מוכר לנו. אם מחקתם ורשמתם מחדש, הנחו את ה-LMS שלכם להסיר ולהוסיף מחדש את הכלי של FastComments כדי שיקבל את ה-client_id החדש.

#### ההפעלה מציגה "Deployment not registered"

הפעלתם את FastComments מהפעלה (deployment) של Brightspace/Moodle/Blackboard שונה מזו שממנה הוא הופעל בפעם הראשונה. FastComments קובע את ה-`deployment_id` בהפעלה הראשונה כבדיקת אבטחה. כדי להוסיף הפעלה חדשה תחת אותו לקוח, פנו לתמיכה - נוסיף את מזהה ההפעלה (Deployment ID) לתצורה.

#### ההפעלה מציגה "Unsupported message_type"

ה-LMS שלח הודעת LTI ש-FastComments אינו תומך בה (למשל `LtiSubmissionReviewRequest`). FastComments תומך רק בזרימות ההפעלה הסטנדרטיות של resource-link ו-deep-linking. פנו אלינו אם אתם זקוקים להוספת סוג הודעה ספציפי.

#### ה-iframe אינו מתאים את גודלו

רוב מערכות ה-LMS מתאימות אוטומטית את גודל ה-iframe של LTI. אם המערכת שלכם לא עושה זאת, בדקו שהגדרות ההפעלה של ה-LMS מאפשרות לכלי לשלוח אירועי postMessage למסגרת האב. FastComments שולח הודעות שינוי גודל הן בסגנון Canvas (`lti.frameResize`) והן לפי תקן IMS (`org.imsglobal.lti.frameResize`).