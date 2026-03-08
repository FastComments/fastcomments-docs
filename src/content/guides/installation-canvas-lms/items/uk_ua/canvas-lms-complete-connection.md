#### Введіть Client ID у FastComments

Поверніться до <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">My Account > Canvas LTI Config</a> у FastComments. Майстер має бути на **Step 2: Connect**.

Вставте **Client ID**, який ви скопіювали з Canvas, у поле **Client ID**. За потреби введіть **Deployment ID**, якщо ваш LMS надає його.

Натисніть **Save & Continue**.

#### Увімкніть інтеграцію

Майстер переходить до **Step 3: Go Live**. Показано зведення вашої конфігурації (назва, platform URL, Client ID та Deployment ID).

Перегляньте деталі, потім натисніть **Enable Integration**, щоб активувати з'єднання LTI.

Після увімкнення майстер показує **Management View**, де ви можете редагувати конфігурацію, переглядати всі LTI URLs або додавати додаткові розгортання.

#### Встановіть External App у Canvas

У Canvas перейдіть до **Admin** > виберіть свій обліковий запис > **Settings** > вкладка **Apps**.

Натисніть **+ App** і налаштуйте:

1. Встановіть **Configuration Type** на **By Client ID**.
2. Вставте **Client ID** з таблиці Developer Keys.
3. Натисніть **Submit**.
4. Підтвердіть встановлення, коли з'явиться запит.

FastComments тепер встановлено на рівні облікового запису та доступний у всіх курсах.