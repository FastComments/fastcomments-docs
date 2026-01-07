אובייקט `Vote` מייצג הצבעה שהושארה על ידי משתמש.

היחס בין תגובות להצבעה מוגדר באמצעות `commentId`.

המבנה עבור אובייקט Vote הוא כדלקמן:

[inline-code-attrs-start title = 'מבנה הצבעה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Vote {
    id: string
    urlId: string
    commentId: string
    userId: string
    direction: 1 | -1
    createdAt: string
}
[inline-code-end]
