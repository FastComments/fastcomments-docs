D2L Brightspace надає Динамічну реєстрацію через інтерфейс адміністратора LTI Advantage. Потрібен доступ адміністратора.

#### Open the Registration Screen

1. Увійдіть у ваш екземпляр Brightspace як адміністратор.
2. Перейдіть до **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Натисніть **Register Tool**. (Пряме посилання: `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Paste the URL

Ви побачите форму реєстрації. Ключове поле — **Tool initiation registration endpoint** (в деяких версіях Brightspace воно позначене як "Tool Initiation Registration URL").

Вставте URL реєстрації FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">get it here</a>) у це поле. Інші поля залиште порожні — вони автоматично заповнюються FastComments під час рукопотискання реєстрації.

Натисніть **Register**.

#### Approve the Tool

Brightspace відкриває спливаюче вікно, яке взаємодіє з FastComments, обмінюється ключами та показує екран підтвердження. Спливаюче вікно закривається автоматично після завершення реєстрації.

Новий інструмент з'явиться у списку інструментів LTI Advantage. За замовчуванням Brightspace позначає нові інструменти як **disabled** — переключіть тумблер на **enabled**, щоб ваші курси могли ним користуватися.

#### Add a Deployment

У Brightspace LTI-інструментам потрібне **deployment** перед тим, як ними можна буде користуватися в курсах:

1. Відкрийте щойно зареєстрований інструмент FastComments.
2. Натисніть **View Deployments** > **New Deployment**.
3. Дайте розгортанню назву (наприклад, "FastComments - All Courses"), виберіть організаційні одиниці, у яких воно має бути доступне, і збережіть.

Після першого запуску через це розгортання FastComments закріплює `deployment_id` у своєму записі конфігурації — наступні запуски з іншого розгортання в межах того ж клієнта будуть відхилені, якщо ви не зареєструєте заново.