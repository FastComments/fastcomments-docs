#### Введите Client ID в FastComments

Вернитесь в <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">Мой аккаунт > Конфигурация Canvas LTI</a> в FastComments. Мастер должен быть на **Step 2: Connect**.

Вставьте **Client ID**, который вы скопировали из Canvas, в поле **Client ID**. По желанию введите **Deployment ID**, если ваша LMS его предоставляет.

Нажмите **Save & Continue**.

#### Включите интеграцию

Мастер переходит к **Step 3: Go Live**. Отображается сводка вашей конфигурации (имя, URL платформы, Client ID и Deployment ID).

Проверьте данные, затем нажмите **Enable Integration** для активации LTI-подключения.

После включения мастер показывает **Management View**, где вы можете редактировать конфигурацию, просматривать все LTI URL-адреса или добавлять дополнительные deployments.

#### Установите внешнее приложение в Canvas

В Canvas перейдите в **Admin** > выберите свой аккаунт > **Settings** > вкладку **Apps**.

Нажмите **+ App** и настройте:

1. Установите **Configuration Type** в **By Client ID**.
2. Вставьте **Client ID** из таблицы Developer Keys.
3. Нажмите **Submit**.
4. Подтвердите установку при появлении запроса.

FastComments теперь установлена на уровне аккаунта и доступна во всех курсах.