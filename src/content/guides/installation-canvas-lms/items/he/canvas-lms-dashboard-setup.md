#### Navigate to Canvas LTI Config

Log in to your FastComments account and go to <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">החשבון שלי &gt; תצורת LTI של Canvas</a>.

#### Create a New LTI Configuration

Check the **הפעל LTI** checkbox. The configuration fields will appear:

- **שם התצורה** - תווית אופציונלית לזיהוי תצורה זו (שימושי כאשר מחברים מספר מופעי Canvas).
- **Platform URL** - כתובת ה-URL של מופע Canvas שלך (e.g. `https://yourschool.instructure.com`). ניתן להשאיר זאת ריקה לעת עתה ולמלא אותה לאחר יצירת ה-Developer Key.
- **Client ID** - השאר ריק לעת עתה. תמלא אותו לאחר יצירת ה-Developer Key ב-Canvas.
- **Deployment ID** - השאר ריק לעת עתה.
- **Comment Style** - בחר בין Comments, Collab Chat, או Both. ראה [סגנונות תגובה](#canvas-lms-commenting-styles) לפרטים.

Click **Add** to create the configuration.

#### Copy the Configuration URLs

After saving, three URLs will appear:

- **Configuration URL** - תדביק אותה ב-Canvas בעת יצירת ה-Developer Key.
- **OIDC Login URL** - משמש את Canvas עבור תהליך ההתחברות של LTI (מוגדר אוטומטית דרך ה-Configuration URL).
- **Launch URL** - נקודת הקצה ש-Canvas קורא לה כאשר סטודנט/ית פותח/ת את FastComments (מוגדר אוטומטית דרך ה-Configuration URL).

Copy the **Configuration URL**. You will need it in the next step.