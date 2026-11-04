הגבול סביב תיבת התגובות מורכב מהמסגרת של אזור הטקסט עצמו ועוד כמה קווים דקים שהווידג\'ט מצייר סביבו. כדי לשנות את הצבע או עיגול הפינות שלו, הגדירו את משתני ה-CSS הבאים במקום לעצב את ה-`textarea` ישירות. המשתנים מעצבים מחדש את כל חלקי הגבול בבת אחת, כך שהפינות והצבעים תמיד יתאימו.

| משתנה | מה הוא משנה | ברירת מחדל |
|---|---|---|
| `--fc-input-border-color` | צבע הגבול | `#bfbfbf` |
| `--fc-input-border-color-focus` | צבע הגבול בזמן שהמשתמש מקליד | `#555` |
| `--fc-input-border-radius` | עיגול הפינות המעוגלות | `11px` |
| `--fc-input-border-start-start-radius` | עיגול הפינה המרובעת השמאלית-עליונה (הימנית-עליונה בשפות מימין לשמאל) | `0` |

הוסיפו את ה-CSS לתיבת **Custom CSS** ב[דף התאמה אישית של הווידג\'ט](https://fastcomments.com/auth/my-account/customize-widget), או העבירו אותו עם האפשרות `customCSS`. צריך להגדיר רק את המשתנים שרוצים לשנות.

אפשר גם להשתמש בכלי העזר **Comment box border** ממש מתחת לתיבת ה-Custom CSS, שכותב את ה-CSS הזה עבורכם.

## שינוי צבע הגבול

[inline-code-attrs-start title = 'צבע הגבול'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## שינוי צבע הגבול בזמן הקלדה

[inline-code-attrs-start title = 'צבע הגבול בזמן הקלדה'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## עיגול כל ארבע הפינות

כברירת מחדל הפינה השמאלית-עליונה מרובעת. הגדירו את שני משתני הרדיוס כדי לעגל את כל ארבע הפינות באופן זהה:

[inline-code-attrs-start title = 'עיגול כל ארבע הפינות'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## פינות מרובעות

[inline-code-attrs-start title = 'פינות מרובעות'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## התאמה למותג שלכם

[inline-code-attrs-start title = 'צבעים ופינות של המותג'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## צבעים שונים במצב כהה

כשהווידג\'ט נמצא במצב כהה יש לו את המחלקה `dark`, ולכן אפשר להגדיר ערכים שונים למצב הכהה:

[inline-code-attrs-start title = 'צבעי גבול במצב כהה'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## למה לא לעצב את אזור הטקסט ישירות?

הווידג\'ט מצייר בעצמו חלק מהגבול של תיבת התגובות, סביב אזור הטקסט. אם תגדירו `border-color` או `border-radius` רק על ה-`textarea`, הקווים האלה יישארו בסגנון ברירת המחדל והגבול ייראה לא אחיד, למשל קו מרובע שחוצה פינה מעוגלת. המשתנים לעיל משנים את שניהם בבת אחת.