#### Sakai

Sakai תומך ב-LTI 1.3 Dynamic Registration בגרסאות עם LTI Advantage. מתוך Administration Workspace:

1. התחברו כמנהל Sakai ופתחו את **Administration Workspace**.
2. בחרו **External Tools** > **Install LTI 1.3 Tool**.
3. הדביקו את כתובת ההרשמה של FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">ניתן להשיג אותה כאן</a>) ושלחו.
4. אשרו את הכלי כאשר תהליך ה-handshake מסתיים.

הכלי יופיע לאחר מכן תחת **External Tools** וניתן להוסיף אותו לאתרים על ידי מנהלי האתרים.

#### Schoology

מופעי Schoology Enterprise תומכים ב-LTI 1.3, אך זמינות ה-Dynamic Registration משתנה בהתאם לפריסה. בדקו עם מנהל החשבון שלכם ב-Schoology.

אם Dynamic Registration אינו זמין במופע ה-Schoology שלכם, תצטרכו להגדיר את האינטגרציה באופן ידני באמצעות נקודות הקצה הבאות:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

לאחר ש-Schoology מספק לכם Client ID ו-Deployment ID, פנו לתמיכה של FastComments כדי לרשום את ההגדרה בטננט שלכם.

#### פלטפורמות LTI 1.3 אחרות

כל LMS העוקב אחר המפרט של IMS LTI 1.3 Advantage אמור לעבוד עם אותה כתובת הרשמה (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">ניתן להשיג אותה כאן</a>). חפשו הגדרה בשם "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" או דומה לכך.

אם הפלטפורמה שלכם תומכת רק בהגדרה ידנית של LTI 1.3, השתמשו בארבע נקודות הקצה המפורטות בסעיף Schoology למעלה ופנו לתמיכה כדי להשלים את התהליך.