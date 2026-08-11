[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

ניתן לנעול את ההערות כך שלא יוכלו להישאר תגובות או הצבעות חדשות על‑ידי הגדרת הדגל readonly ל‑true.

התגובות גם לא יוכלו להיות ערוכות או נמחקות.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

ניתן להתאים זאת ללא קוד, בעמוד התאמת הווידג'ט, עבור תחום שלם או דף:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; alt='הגדרת מניעת תגובות חדשות בעמוד התאמת הווידג\'ט, אשר נועלת שרשור עבור תחום או דף'; title='Making The Comment Thread Readonly' app-screenshot-end]

## Update!

מאז נובמבר 2022, ניתן לנעול או לשחרר שרשורים **בזמן אמת** על‑ידי מנהלים ומנהלים משניים דרך תפריט שלוש הנקודות מעל אזור התגובה.

זה ימנע תגובות חדשות, בעוד שההצבעה עדיין אפשרית, והמשתמשים יכולים למחוק את תגובותיהם אם ירצו, בעוד ש‑`readonly` אינו מאפשר את הדברים האלה. 

זה תואם לשדה `isClosed` ב‑API של `Page`.

---