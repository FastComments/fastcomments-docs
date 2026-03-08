#### Перейдите в Canvas LTI Config

Войдите в свой аккаунт FastComments и перейдите в <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">My Account > Canvas LTI Config</a>.

#### Создайте новую конфигурацию LTI

Установите флажок **Enable LTI**. Появятся поля конфигурации:

- **Configuration Name** - необязательная метка для идентификации этой конфигурации (полезно, если вы подключаете несколько экземпляров Canvas).
- **Platform URL** - URL вашей инстанции Canvas (например, `https://yourschool.instructure.com`). Вы можете оставить это поле пустым и заполнить его после создания Developer Key.
- **Client ID** - оставьте это поле пустым пока. Вы заполните его после создания Developer Key в Canvas.
- **Deployment ID** - оставьте это поле пустым пока.
- **Comment Style** - выберите между Comments, Collab Chat или Both. Подробности см. в разделе [Commenting Styles](#canvas-lms-commenting-styles).

Нажмите **Add**, чтобы создать конфигурацию.

#### Скопируйте URL-адреса конфигурации

После сохранения появятся три URL-адреса:

- **Configuration URL** - вы вставите его в Canvas при создании Developer Key.
- **OIDC Login URL** - используется Canvas для процесса входа LTI (автоматически настраивается через Configuration URL).
- **Launch URL** - конечная точка, к которой Canvas обращается, когда студент открывает FastComments (автоматически настраивается через Configuration URL).

Скопируйте **Configuration URL**. Он понадобится вам на следующем шаге.