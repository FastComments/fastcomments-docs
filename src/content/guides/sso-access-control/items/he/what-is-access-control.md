With FastComments SSO Access Control, sometimes referred to as RBAC, users can be restricted to only access certain pages, or comment threads. Additionally,
users can only `@mention` each other in the same group.

## בפירוט

We can place `Users` and optionally `Pages` into groups.

When `Users` are placed into groups, and `Limit Comments by SSO User Groups` is enabled in Widget Settings, then users
will only see comments from users in their same groups and will only be able to `@mention` users in the same groups.

Additionally, `Pages` can be placed into groups, and then users can only access comments for pages they have access to.

We call this "קבוצות ברמת משתמש" verses "קבוצות ברמת דף".

So which one is right for you?

#### השתמשו בקבוצות ברמת משתמש אם...

- אתם רוצים להשתמש בערך ה-`urlId` זהה (כתובת הדף, או מזהה המאמר), אך להגביל תגובות לפי קבוצה.
- לדוגמה, אתם רוצים שיהיו קבוצות "משתמש חדש" ו-"משתמש ותיק", והן לא צריכות לראות לעולם את תגובות זו של זו באותם דפים.

#### השתמשו בקבוצות ברמת דף אם...

- לקבוצות שלכם יש דפים ספציפיים.
- לדוגמה, משתמשים בקבוצה "Public Pages" לא צריכים לעולם לצפות במאמרים של "Top Secret" articles.