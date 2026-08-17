Each instance of the comment widget is isolated. Because of this, FastComments inherently supports more than one instance per page, or multiple
instances pointing to the same chat thread.

In the case of the VanillaJS library for example, you simply have to tie the comment widget to different DOM nodes. If you want to simply
update the current thread on the page, see [Switching Comment Threads Without Reloading The Page](guide-customizations-and-configuration.html#switching-comment-threads);

### סנכרון מצב האימות בין מופעים מרובים

בואו נסקור את הדוגמה של יישום עמוד יחיד מותאם אישית שהוא רשימת שאלות נפוצות עם שרשור ההערות שלהן.

במקרה זה, יש לנו מספר מופעים של FastComments ב‑DOM בו‑זמנית.

זה בסדר, אך זה מציב כמה אתגרים בחוויית המשתמש.

שקלו את הזרימה הזו:

1. המשתמש מבקר בעמוד עם רשימת שאלות, שלכל אחת יש וידג'ט ההערות שלה.
2. המשתמש מזין שם משתמש וכתובת אימייל ומשאיר שאלה באחד מהשרשורים.
3. הוא רואה פריט FAQ נוסף שיש לו שאלה לגביו.
4. הוא חוזר להוסיף תגובה. האם הוא צריך להזין שוב את האימייל ושם המשתמש שלו?

במקרה זה, FastComments מטפל בסנכרון מצב האימות בין מופעי הווידג'ט עבורך. בצעד הרביעי, המשתמש כבר יהיה מאומת זמנית מכיוון שהזין את שם המשתמש והאימייל שלו באותו עמוד.