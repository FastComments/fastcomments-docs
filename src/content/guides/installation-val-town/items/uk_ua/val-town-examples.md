Чотири публічних валів, які ви можете реміксити, кожен охоплює одну частину цього посібника.

**[Blog with comments](https://www.val.town/x/fastcomments/blog-with-comments)** ([live](https://fastcomments-blog.val.run)) — це Markdown‑блог з гілкою під кожним постом і підрахунком коментарів у масиві на індексі. Він працює відразу після реміксу, і одна змінна середовища вказує його на ваш власний обліковий запис.

**[SSO demo](https://www.val.town/x/fastcomments/sso-demo)** ([live](https://fastcomments-sso.val.run)) входить відвідувача за допомогою їхнього облікового запису Val Town і передає цю ідентичність віджету, тому другий вхід не потрібен.

**[Webhook receiver](https://www.val.town/x/fastcomments/webhook-receiver)** ([live](https://fastcomments-webhooks.val.run)) перевіряє HMAC‑підпис кожної доставки і зберігає події в SQLite. Має кнопку, яка підписує тестове навантаження і надсилає його самому собі, тож ви можете спостерігати успішну верифікацію перед налаштуванням реального вебхука.

**[Agent skills](https://www.val.town/x/fastcomments/skills)** ([live](https://fastcomments-skills.val.run)) — це бібліотека навичок агента FastComments, що охоплює віджет, SSO, REST API, модерацію та міграцію з Disqus. Реміксуйте її, і агент Val Town, Townie, автоматично підхоплює навички з `skills/`, тож ваш агент знає, як підключити коментарі без копіювання документації в чат.

Ті ж навички можна встановити будь-де за допомогою `npx skills add fastcomments/skills`.