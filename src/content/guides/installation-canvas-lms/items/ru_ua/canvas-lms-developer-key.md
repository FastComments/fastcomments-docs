#### Open Developer Keys in Canvas

Войдите в Canvas под учетной записью администратора. Перейдите в **Admin** (в левой боковой панели) > выберите свой аккаунт > **Developer Keys**.

#### Create an LTI Developer Key

Нажмите **+ Developer Key** и выберите **LTI Key**.

В форме конфигурации:

1. В поле **Redirect URIs** (слева) вставьте **Launch URL** со страницы настройки FastComments.
2. Справа установите **Method** в **Enter URL**.
3. Вставьте **Configuration URL**, который вы скопировали из FastComments, в поле **JSON URL**.
4. Canvas автоматически загрузит конфигурацию LTI.
5. Дайте ключу имя (например "FastComments").
6. Нажмите **Save**.

#### Enable the Developer Key

После сохранения новый ключ появится в таблице Developer Keys со статусом **State**, установленным в **OFF**. Нажмите переключатель, чтобы установить его в **ON**. Canvas может запросить подтверждение — нажмите **Allow**, чтобы включить ключ.

#### Copy the Client ID

В таблице Developer Keys в колонке **Details** отображается числовой **Client ID** (например `17000000000042`). Скопируйте это число — оно понадобится вам на следующем шаге.

---