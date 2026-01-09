[related-parameter-start name = 'maxReplyDepth'; type = 'number'; related-parameter-end]

כברירת מחדל, FastComments מאפשר קינון בלתי מוגבל של תגובות, ויוצר מבנה פתיל (thread) שבו משתמשים יכולים להשיב לתשובות ללא הגבלה.

האפשרות maxReplyDepth מאפשרת לך להגביל עד כמה עמוקים יכולים להיות שרשורי התשובות. כאשר מגיעים לעומק המקסימלי, המשתמשים כבר לא יראו כפתור תגובה על התגובות ברמה זו.

[code-example-start config = {maxReplyDepth: 2}; linesToHighlight = [6]; title = 'Limiting Reply Depth to 2 Levels'; code-example-end]

With maxReplyDepth set to 2:
- Users can comment at the top level (depth 0)
- Users can reply to top-level comments (depth 1)
- Users can reply to those replies (depth 2)
- No further replies are allowed beyond depth 2

Setting to 1 would only allow replies to top-level comments, creating a flatter discussion structure.

Setting maxReplyDepth to 0 would disable all replies, allowing only top-level comments. If not specified, replies can be nested without limit.