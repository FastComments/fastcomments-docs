D2L Brightspace омогућава Dynamic Registration преко LTI Advantage административног интерфејса. Биће вам потребан приступ администратора.

#### Отворите екран за регистрацију

1. Пријавите се у вашу Brightspace инстанцу као администратор.
2. Идите на **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Кликните **Register Tool**. (Директан URL је `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Налепите URL

Видећете формулар за регистрацију. Кључно поље је **Tool initiation registration endpoint** (неке верзије Brightspace-а то означавају као "Tool Initiation Registration URL").

Налепите FastComments registration URL у то поље. Оставите остала поља празна - FastComments ће их само-попунити током регистрационог руковања.

Кликните **Register**.

#### Одобрите алат

Brightspace отвара popup који комуницира са FastComments, размењује кључеве и приказује потврдни екран. Popup се затвара сам кад регистрација буде завршена.

Нови алат се појављује у вашем списку LTI Advantage алата. По подразумеваној поставци Brightspace означава нове алате као **disabled** - укључите прекидач на **enabled** да би ваши курсеви могли да га користе.

#### Додајте Deployment

У Brightspace-у, LTI алати требају имати **deployment** пре него што могу да се користе у курсевима:

1. Отворите новорегистровани FastComments алат.
2. Кликните **View Deployments** > **New Deployment**.
3. Дајте deployment-у име (нпр. "FastComments - All Courses"), изаберите организационе јединице у којима треба да буде доступан и сачувајте.

Након првог покретања преко овог deployment-а, FastComments повезује `deployment_id` са својим записом конфигурације - накнадна покретања из другог deployment-а у оквиру истог клиента биће одбијена осим ако се поново не региструјете.