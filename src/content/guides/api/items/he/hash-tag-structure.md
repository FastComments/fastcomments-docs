אובייקט `HashTag` מייצג תגית שיכולה להיות מושארת על ידי משתמש. האשטאגים יכולים לשמש לקישור לתוכן חיצוני או
לקשר תגובות קשורות יחד.

המבנה עבור אובייקט `HashTag` הוא כדלקמן:

[inline-code-attrs-start title = 'מבנה האשטאג'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTag {
    /** Should start with the "#" or desired character. **/
    tag: string
    /** An optional URL that the hashtag can point to. Instead of filtering comments by hashtag, the UI will redirect to this upon click. **/
    url?: string
    /** READONLY **/
    createdAt: string
}
[inline-code-end]

הערות:

- בחלק מנקודות הקצה של ה-API תראה שההאשטאג משמש ב-URL. זכור לקודד URI את הערכים. לדוגמה, `#` צריך להיות מיוצג כ-`%23`.
- חלק מהשדות הללו מסומנים כ-`READONLY` - אלה מוחזרים על ידי ה-API אבל לא ניתן להגדיר אותם.
