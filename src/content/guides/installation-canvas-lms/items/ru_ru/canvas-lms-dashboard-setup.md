#### Перейдите в Конфигурацию Canvas LTI

Войдите в свой аккаунт FastComments и перейдите на <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">Мой аккаунт > Конфигурация Canvas LTI</a>.

#### Создание новой конфигурации LTI

Установите флажок **Включить LTI**. Появятся поля конфигурации:

- **Configuration Name** - необязательная метка для идентификации этой конфигурации (полезно, если вы подключаете несколько экземпляров Canvas).
- **Platform URL** - URL вашей инстанции Canvas (например, `https://yourschool.instructure.com`). Вы можете оставить это поле пустым сейчас и заполнить после создания Developer Key.
- **Client ID** - пока оставьте пустым. Вы заполните его после создания Developer Key в Canvas.
- **Deployment ID** - пока оставьте пустым.
- **Comment Style** - выберите между Comments, Collab Chat или Both. Смотрите [Commenting Styles](#canvas-lms-commenting-styles) для подробностей.

Нажмите **Добавить** для создания конфигурации.

#### Скопируйте URL-адреса конфигурации

После сохранения появятся три URL:

- **URL конфигурации** - вы вставите это в Canvas при создании Developer Key.
- **OIDC Login URL** - используется Canvas для процесса входа LTI (автоматически настраивается через Configuration URL).
- **Launch URL** - endpoint, который Canvas вызывает, когда студент открывает FastComments (автоматически настраивается через Configuration URL).

Скопируйте **URL конфигурации**. Он понадобится вам на следующем шаге.