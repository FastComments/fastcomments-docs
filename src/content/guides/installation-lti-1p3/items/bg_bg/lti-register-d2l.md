D2L Brightspace exposes Dynamic Registration through the LTI Advantage admin interface. You will need administrator access.

#### Отворете екрана за регистрация

1. Влезте в своя Brightspace инстанс като администратор.
2. Отидете на **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Кликнете **Register Tool**. (Директният URL е `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Поставете URL адреса

Ще видите регистрационна форма. Ключовото поле е **Tool initiation registration endpoint** (някои версии на Brightspace го означават като "Tool Initiation Registration URL").

Поставете FastComments регистрационния URL в това поле. Оставете останалите полета празни - те се попълват автоматично от FastComments по време на регистрационния handshake.

Кликнете **Register**.

#### Одобрете инструмента

Brightspace отваря popup, който комуникира с FastComments, разменя ключове и показва екран за потвърждение. Popup прозорецът се затваря сам, когато регистрацията завърши.

Новият инструмент се появява в списъка с LTI Advantage инструменти. По подразбиране Brightspace маркира новите инструменти като **disabled** - превключете тумблера на **enabled**, за да могат курсовете ви да го използват.

#### Добавете Deployment

В Brightspace, LTI инструментите се нуждаят от **deployment** преди да могат да бъдат използвани в курсове:

1. Отворете току-що регистрирания FastComments инструмент.
2. Кликнете **View Deployments** > **New Deployment**.
3. Дайте име на deployment-а (напр. "FastComments - All Courses"), изберете организационните единици, в които да бъде наличен, и запазете.

След първото стартиране чрез този deployment, FastComments фиксира `deployment_id` в своя запис за конфигурация - последващи стартирания от друг deployment под същия клиент ще бъдат отхвърлени, освен ако не регистрирате отново.