D2L Brightspace омогућава Dynamic Registration преко LTI Advantage администраторског интерфејса. Биће вам потребан администраторски приступ.

#### Отворите екран за регистрацију

1. Пријавите се у вашу Brightspace инстанцу као администратор.
2. Идите на **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Кликните **Register Tool**. (The direct URL is `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Залепите URL

Видећете формулар за регистрацију. Кључно поље је **Tool initiation registration endpoint** (неке верзије Brightspace-а га означавају као "Tool Initiation Registration URL").

Залепите FastComments registration URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">преузмите га овде</a>) у то поље. Оставите остала поља празним - FastComments ће их аутоматски попунити током регистрационог руковања.

Кликните **Register**.

#### Одобрите алат

Brightspace отвара поп-ап који комуницира са FastComments, размењује кључеве и приказује потврдни екран. Поп-ап се сам затвара када регистрација буде завршена.

Нови алат се појављује на вашој листи LTI Advantage алата. По подразумевану Brightspace означава нове алате као **disabled** - померите прекидач у положај **enabled** како би ваши курсеви могли да га користе.

#### Додајте deployment

У Brightspace-у, LTI алати захтевају **deployment** пре него што могу да се користе у курсевима:

1. Отворите недавно регистровани FastComments алат.
2. Кликните **View Deployments** > **New Deployment**.
3. Дајте deployment-у име (нпр. "FastComments - All Courses"), изаберите организационе јединице у којима треба да буде доступан и сачувајте.

Након првог покретања преко овог deployment-а, FastComments везује `deployment_id` за свој запис конфигурације - будућа покретања из другог deployment-а под истим клијентом биће одбијена осим ако се поново не региструјете.