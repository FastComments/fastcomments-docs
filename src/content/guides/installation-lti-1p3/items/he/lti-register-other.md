#### Sakai

Sakai תומך ב‑LTI 1.3 וב‑Dynamic Registration במהדורות הכוללות LTI Advantage. מתוך ה‑Administration Workspace:

1. התחבר כ‑מנהל של Sakai ופתח את ה‑**Administration Workspace**.
2. בחר **External Tools** > **Install LTI 1.3 Tool**.
3. הדבק את כתובת ההרשמה של FastComments והגש.
4. אשר את הכלי כאשר ה‑handshake יסתיים.

הכלי יופיע אז תחת **External Tools** וניתן להוספה לאתרים על‑ידי מנהליהם.

#### Schoology

מופעי Schoology Enterprise תומכים ב‑LTI 1.3, אך זמינות ה‑Dynamic Registration משתנה לפי פריסה. בדוק זאת מול מנהל החשבון של Schoology שלך.

אם ה‑Dynamic Registration אינו זמין ב‑Schoology שלך, תצטרך להגדיר את האינטגרציה באופן ידני באמצעות נקודות הקצה הבאות:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

לאחר שתקבל מ‑Schoology Client ID ו‑Deployment ID, פנה לתמיכת FastComments כדי לרשום את התצורה ב‑tenant שלך.

#### Other LTI 1.3 Platforms

כל LMS שעומד בתקן IMS LTI 1.3 Advantage אמור לעבוד עם אותה כתובת הרשמה. חפש הגדרה המסומנת כ‑"Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" או דומה לכך.

אם הפלטפורמה שלך תומכת רק בהגדרה ידנית של LTI 1.3, השתמש בנקודות הקצה הארבע המפורטות בסעיף Schoology לעיל ופנה לתמיכה כדי להשלים את ההגדרה.