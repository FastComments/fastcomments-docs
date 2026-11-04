## דוגמאות Zaps

מספר תהליכי עבודה שהקמתם לוקחת דקות ספורות.

**קבלת התראה על תגובות חדשות.** New Comment, ואז Slack "Send Channel Message" או Discord "Send Channel Message". מפו את שם המגיב, טקסט התגובה וכתובת ה-URL של העמוד לתוך ההודעה. הוסיפו את מסנן הדומיין כדי לקבל התראה בערוץ אחר לכל אתר.

**שמירת יומן של כל תגובה.** New Comment, ואז Google Sheets "Create Spreadsheet Row". הוסיפו Deleted Comment כ-Zap שני שמוסיף שורה עם מזהה התגובה, כך שהגיליון משמש גם כראיה ביקורתית.

**שליחת אימייל למחבר כאשר תגובה מאושרת.** Updated Comment עם מסנן Zapier על Approved שווה ל-true, ואז Gmail "Send Email". מכיוון ש-Updated Comment מופעל בכל שינוי, המסנן הוא מה שמבטיח שה-Zap יגיב רק לאישורים.

**הוספת מגיבים ל-CRM או לרשימת הדיוור שלכם.** New Comment, ואז HubSpot "Create or Update Contact" או Mailchimp "Add or Update Subscriber" באמצעות האימייל של המגיב. כבדו את מדיניות הפרטיות שלכם ואת החוק המקומי לפני הוספת כל אדם לרשימת שיווק.

**יצירת תגובה מטופס.** Typeform או Google Forms "New Response", ואז FastComments Create Comment עם מזהה ה-URL של העמוד בו האתר שלכם משתמש עבור המלצות. השאירו את Approved לא מסומן כדי לסקור כל תגובה לפני שהיא מופיעה.

**פרסום הכרזות בפיד.** RSS by Zapier "New Item in Feed", ואז Create Feed Post עם הכותרת, התוכן והקישור של הפריט.

**הקצאת חברים כמשתמשי SSO.** Memberstack, Memberful, או webhook משלכם, ואז Find SSO User ולאחריו Create SSO User במצב "find or create".

**העלאת תגובות מדווחות לטיפול.** Updated Comment, מסונן על ספירת דגלים גבוהה מאפס, ואז Trello "Create Card" או Linear "Create Issue" עם מזהה התגובה וקישור לעמוד הפיקוח.

**פרסום עמודים כשהם עולים לאוויר.** WordPress או Ghost "New Post", ואז Create Page עם ה-URL של הפוסט, כך שהעמוד יופיע ברשימה ויוגבל לפני התגובה הראשונה.

**ארכוב תגובות שנמחקו.** Deleted Comment, ואז Airtable "Create Record" עם התגובה המלאה לצורך שמירה לתאימות רגולטורית.