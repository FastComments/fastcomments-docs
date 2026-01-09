[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments נבנה כדי לאפשר התאמה אישית, והגופן שבו משתמשים הווידג'טים שלנו אינו יוצא מן הכלל.

כברירת מחדל, FastComments משתמש ב-`system font stack` כדי להיראות הטוב ביותר על מגוון רחב של מכשירים.

כדי להגדיר גופנים משלך, ראה את [תיעוד ה-CSS המותאם](/guide-customizations-and-configuration.html#custom-css).

שם תמצא דרך להגדיר CSS מותאם, שיאפשר לך לקבוע את הגופנים הרצויים.

#### כיצד להגדיר את הגופן

כדי לעקוף את הגופן, אנו ממליצים להגדיר את ה-CSS שלך באמצעות הסלקטורים `.fast-comments, textarea`. לדוגמה:

[inline-code-attrs-start title = 'דוגמה לגופן חיצוני מותאם'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap');
.fast-comments, textarea {
    font-family: 'Roboto', sans-serif;
}
[inline-code-end]

---