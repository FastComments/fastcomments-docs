#### Sakai

Sakai תומכת בהרשמה דינמית של LTI 1.3 בגרסאות עם LTI Advantage. מתוך **מרחב הניהול**:

1. התחבר כמנהל ב‑Sakai ופתח את **מרחב הניהול**.
2. בחר **כלים חיצוניים** > **התקן כלי LTI 1.3**.
3. הדבק את כתובת הרישום של FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">קבלו אותה כאן</a>) והגש.
4. אשר את הכלי כאשר תהליך החיבור יסתיים.

הכלי יופיע אז תחת **כלים חיצוניים** וניתן להוספה לאתרים על ידי המנהלים שלהם.

#### Schoology

מופעי Schoology Enterprise תומכים ב‑LTI 1.3, אבל זמינות ההרשמה הדינמית משתנה לפי פריסה. בדקו עם מנהל החשבון שלכם ב‑Schoology.

אם הרשמה דינמית אינה זמינה ב‑Schoology שלכם, יהיה עליכם להגדיר את האינטגרציה באופן ידני באמצעות נקודות הקצה הבאות:

- **כתובת כניסה OIDC**: `https://fastcomments.com/lti/v1p3/login`
- **כתובת הקישור היעד**: `https://fastcomments.com/lti/v1p3/launch`
- **כתובת מערך המפתחות הציבוריים (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **כתובות הפניה**: `https://fastcomments.com/lti/v1p3/launch`

לאחר ש‑Schoology תספק לכם Client ID ו‑Deployment ID, פנו לתמיכת FastComments כדי לרשום את התצורה ב‑tenant שלכם.

#### Other LTI 1.3 Platforms

כל LMS שעומד במפרט IMS LTI 1.3 Advantage אמור לעבוד עם אותה כתובת רישום (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">קבלו אותה כאן</a>). חפשו הגדרה שכותרתה "הרשמה דינמית", "כתובת רישום כלי", "נקודת קצה לרישום יזום של כלי", או דומה.

אם הפלטפורמה שלכם תומכת רק בהגדרה ידנית של LTI 1.3, השתמשו בארבע נקודות הקצה המפורטות בסעיף Schoology לעיל ופנו לתמיכה כדי להשלים את התהליך.