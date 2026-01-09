---
[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

ניתן לנעול את יכולת ההערה כך שלא ניתן יהיה להשאיר תגובות או הצבעות חדשות על-ידי הגדרת הדגל readonly ל-true.

כמו כן, לא ניתן יהיה לערוך או למחוק תגובות קיימות.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

ניתן להתאים זאת ללא קוד, בדף התאמת הווידג'ט, עבור דומיין שלם, או עבור דף בודד:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; title='Making The Comment Thread Readonly' app-screenshot-end]

## עדכון!

נכון לנובמבר 2022, ניתן לנעול או לשחרר שרשורי תגובות **בזמן אמת** על-ידי מנהלים ומודרטורים דרך תפריט שלוש הנקודות שמעל אזור התגובה.

זה ימנע תגובות חדשות, תוך שהוא עדיין מאפשר הצבעות, ומאפשר למשתמשים למחוק את תגובותיהם אם ירצו, בעוד ש-`readonly` לא מאפשר את הדברים הללו.

זה מקביל לשדה `isClosed` ב-API של `Page`.

---