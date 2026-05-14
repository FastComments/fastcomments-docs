D2L Brightspace exposes Dynamic Registration through the LTI Advantage admin interface. Вам потребуется доступ администратора.

#### Open the Registration Screen

1. Sign in to your Brightspace instance as an admin.
2. Navigate to **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Click **Register Tool**. (The direct URL is `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Paste the URL

Откроется форма регистрации. Ключевое поле — **Tool initiation registration endpoint** (в некоторых версиях Brightspace оно помечено как "Tool Initiation Registration URL").

Вставьте URL регистрации FastComments в это поле. Оставьте остальные поля пустыми — они будут автоматически заполнены FastComments в ходе рукопожатия при регистрации.

Click **Register**.

#### Approve the Tool

Brightspace откроет всплывающее окно, которое обменяется данными с FastComments, обменяет ключи и покажет экран подтверждения. Всплывающее окно закроется автоматически по завершении регистрации.

Новый инструмент появится в списке инструментов LTI Advantage. По умолчанию Brightspace помечает новые инструменты как **disabled** — включите переключатель в положение **enabled**, чтобы ваши курсы могли им пользоваться.

#### Add a Deployment

В Brightspace LTI-инструментам требуется **deployment** прежде чем их можно будет использовать в курсах:

1. Open the newly-registered FastComments tool.
2. Click **View Deployments** > **New Deployment**.
3. Give the deployment a name (e.g. "FastComments - All Courses"), pick the org units it should be available in, and save.

After the first launch through this deployment, FastComments pins the `deployment_id` to its configuration record - subsequent launches from a different deployment under the same client will be rejected unless you re-register.