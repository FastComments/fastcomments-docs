FastComments מספקת חמישה תבניות התחלה כדי שלא תצטרכו לכתוב סוכן עובד מאפס. ניתן להגיע אליהן מ[דף סוכני ה-AI](https://fastcomments.com/auth/my-account/ai-agents) על ידי לחיצה על **Browse templates**.

כאשר בוחרים תבנית:

1. הסוכן נוצר עם **Status: Dry Run** ושם פנימי המבוסס על התבנית (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`, `gaslight_detector`). אם השם כבר תפוס בטננט שלכם, יתווסף סיומת מספרית.
2. אתם מגיעים ישירות לטופס העריכה עם הכל ממולא מראש - הפרומפט, הטריגרים, הפעולות המותרות וכל ספי ההפעלה. באנר לרוחב החלק העליון אומר "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready."
3. שום דבר עדיין לא מופעל. הסוכן לא יפעל עד שתשמרו ותשאירו את מצב ה-dry-run דלוק (לצפייה) או תעברו ל-Enabled.

### חמש התבניות

- **[Moderator](#template-moderator)** - סוקר תגובות חדשות ומדוגלות, מזהיר עבריינים בפעם הראשונה, ומטפל בהרחקה (ban) רק לאחר אזהרה. מופעל על תגובות חדשות ועל חציית סף דיגולים (סף דיגולים ברירת מחדל: 3). כלים מותרים: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - מגיב בחום למגיבים בפעם הראשונה עם פתיחה קצרה ואישית. מופעל על new-user-first-comment. כלי מותר: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - נועץ תגובות ראשיות מהותיות ברגע שהן חוצות סף הצבעות (ברירת מחדל: 10), כאשר קודם מבטל נעיצה של התגובה הנעוצה הקודמת. מופעל על חציית סף הצבעות. כלים מותרים: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - מפרסם סיכום נייטרלי בפסקה אחת בשרשורים ארוכים לאחר השהיה, ואז נועץ אותו. מופעל על תגובות חדשות עם דחייה של 30 דקות כדי שהשרשור יתייצב לפני הסיכום. כלים מותרים: `write_comment`, `pin_comment`, `unpin_comment`.

- **[Gaslight Detector](#template-gaslight-detector)** - עוקב אחר עריכות תגובות לשכתובים באמצע שרשור שמעוותים תשובות, משחזר את הטקסט המקורי ושולח הודעה פרטית למחבר. מופעל על עריכות תגובות. כלים מותרים: `edit_comment`, `warn_user`, `send_dm`.

### התאמה אישית של תבנית

התבניות הן נקודות פתיחה, לא חוזים. מצופה מכם:

- לכוונן את **Initial prompt** כך שיתאים לקול הקהילה שלכם.
- להוסיף או להסיר **Triggers** בהתאם לתדירות הרצויה של הסוכן.
- להוסיף **Approvals** לכל פעולה רגישה - אנו ממליצים בחום להציב אישור על `ban_user` בתבניות מסוג מנחה.
- להוסיף **Community guidelines** כדי שהסוכן ייישם את המדיניות הכתובה שלכם באופן עקבי. ראו [Community Guidelines](#community-guidelines).
- להגדיר **Budgets** פר-סוכן המתאימים למספר הטריגרים הצפוי.

התבנית היא רק אמצעי שממלא מראש ברירות מחדל הגיוניות; ברגע שנשמר, הסוכן שייך לכם.