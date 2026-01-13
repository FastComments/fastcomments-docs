---
ההרחבה הקטנה ביותר האפשרית תהיה:

[inline-code-attrs-start title = 'תוסף פשוט'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

לצורך הדוגמה, שמור קובץ זה כ-`my-extension.js`, והפוך אותו לזמין ב-`https://example.com/my-extension.min.js`.

תוסף זה אינו עושה דבר; בעת טעינה בלבד הוא מאחזר את אובייקט ה-`Extension` שנוצר על ידי ספריית ההערות המרכזית.

אובייקט ה-`Extension` הוא סינגלטון ואינו משותף עם תוספים אחרים.

בהמשך, כדי לטעון את התוסף שלנו, עלינו להודיע על כך לווידג'ט ההערות. לדוגמה:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Using a Custom Extension'; code-example-end]

לדוגמות פונקציונליות, ראה את הסעיף הבא.

---