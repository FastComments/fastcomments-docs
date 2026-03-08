#### Введите Client ID в FastComments

Вернитесь на <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">My Account > Canvas LTI Config</a> в FastComments. Мастер должен быть на **Step 2: Connect**.

Вставьте **Client ID**, скопированный из Canvas, в поле **Client ID**. При желании введите **Deployment ID**, если ваша LMS предоставляет его.

Нажмите **Save & Continue**.

#### Включение интеграции

Мастер переходит к **Step 3: Go Live**. Отображается сводка вашей конфигурации (name, platform URL, Client ID, and deployment ID).

Просмотрите детали, затем нажмите **Enable Integration**, чтобы активировать LTI-подключение.

После включения мастер показывает **Management View**, где вы можете редактировать вашу конфигурацию, просматривать все LTI URLs или добавлять дополнительные deployments.

#### Установите внешнее приложение в Canvas

В Canvas перейдите в **Admin** > выберите ваш аккаунт > **Settings** > вкладка **Apps**.

Нажмите **+ App** и настройте:

1. Установите **Configuration Type** в **By Client ID**.
2. Вставьте **Client ID** из таблицы Developer Keys.
3. Нажмите **Submit**.
4. Подтвердите установку, когда появится запрос.

FastComments теперь установлен на уровне аккаунта и доступен во всех курсах.