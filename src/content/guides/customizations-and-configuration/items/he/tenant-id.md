[related-parameter-start name = 'tenantId'; type = 'string'; related-parameter-end]

יתכן שתגלה כי הווידג'ט לתגובות ניתן לשימוש עם מזהה שוכר (Tenant ID) של "demo", למשל:

[code-example-start config = {tenantId: 'demo'}; linesToHighlight = [5]; title = 'Demo Tenant ID'; code-example-end]

זה מיועד רק כדי להתנסות ולשחק עם הווידג'ט לתגובות. בסביבת ייצור, תעביר את מזהה השוכר שלך, כך:

[code-example-start config = {tenantId: '{{{ExampleTenantId}}}'}; linesToHighlight = [5]; title = 'Defining Your Tenant ID'; code-example-end]

מזהה השוכר שלך כבר מיושם על הווידג'ט לתגובות ב-<a href="https://fastcomments.com/auth/my-account/get-acct-code" target="_blank">קטע הקוד בחשבונך</a>.

תוכל גם למצוא את מזהה השוכר שלך ולנהל את מפתחות ה-API שלך [בעמוד האישורים של ה-API](https://fastcomments.com/auth/my-account/api-secret).

מרגע זה, אם אתה מחובר ל-FastComments, דוגמאות הקוד ישתמשו במזהה השוכר האמיתי שלך (אם אתה מחובר ב־https://fastcomments.com).

---