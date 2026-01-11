עבור מפתחים שאולי אינכם רוצים שהם יהיו `Administrators`, שקלו ליצור משתמש `Administrator`
עם ההרשאות הבאות:

1. Analytics Admin
2. Customizations Admin
3. Data Management Admin
4. Comment Moderation Admin
5. API/SSO Admin

סט הרשאות זה ייתן למפתח כל מה שהוא צריך כדי להגדיר את FastComments כמו גם
הנראות בתוך המערכת הנדרשת כדי לוודא שהיא פועלת.

ההסבר לגבי הרשאות אלה הוא כדלקמן:

1. **Analytics Admin**: ניתן להשתמש בו כדי לנפות תקלות בשימוש ב-FastComments.
2. **Customizations Admin**: זה יהיה דרוש כדי להחיל עיצוב מותאם אישית על הווידג'ט של התגובות.
3. **Data Management Admin**: זה יהיה דרוש לניהול ייבוא וייצוא, ולהגדרת webhooks.
4. **Comment Moderation Admin**: זה יהיה דרוש כדי לראות נתוני תגובות, לפחות במהלך ההגדרה.
5. **API/SSO Admin**: זה יאפשר להם לשלוף את API keys ישירות מהפלטפורמה שלנו. אנחנו רואים
שזה בטוח יותר מאשר `Administrator` שמעתיק אותו עבורם ושולח את API Secret בדוא"ל, ש
   עלול לא להיות מאובטח מאוד.