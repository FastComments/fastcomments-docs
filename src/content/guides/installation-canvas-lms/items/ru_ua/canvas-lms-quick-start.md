1. Войдите в FastComments и перейдите в <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">My Account > Canvas LTI Config</a>.
2. Введите **Configuration Name** и ваш **Platform URL** (например `https://yourschool.instructure.com`), затем нажмите **Create Configuration**. Мастер переходит к Step 2 и показывает ваш **Configuration URL**.
3. В Canvas перейдите в **Admin > Developer Keys > + Developer Key > LTI Key**. Установите **Method** в "Enter URL" и вставьте Configuration URL. Сохраните ключ и установите его **State** в **ON**.
4. Скопируйте номер **Client ID** из таблицы Developer Keys в Canvas. Вернувшись в FastComments, вставьте его в поле **Client ID** и нажмите **Save & Continue**.
5. Просмотрите сводку конфигурации и нажмите **Enable Integration**, чтобы активировать интеграцию.
6. В своём курсе Canvas перейдите в **Settings > Navigation**, найдите **FastComments** и включите его. Комментарии появятся в навигации курса.