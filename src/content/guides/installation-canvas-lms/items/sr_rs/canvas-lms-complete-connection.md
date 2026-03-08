#### Унесите Client ID у FastComments

Вратите се на <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">My Account > Canvas LTI Config</a> у FastComments. Чаробњак треба да буде на **Step 2: Connect**.

Налепите **Client ID** који сте копирали из Canvas у поље **Client ID**. Опционално унесите **Deployment ID** ако вам LMS пружа такав.

Кликните **Save & Continue**.

#### Омогућите интеграцију

Чаробњак напредује до **Step 3: Go Live**. Приказан је резиме ваше конфигурације (име, platform URL, Client ID, и Deployment ID).

Прегледајте детаље, затим кликните **Enable Integration** да бисте активирали LTI везу.

Након омогућавања, чаробњак приказује **Management View** где можете уредити вашу конфигурацију, видети све LTI URLs, или додати додатне deployment-ове.

#### Инсталирајте External App у Canvas

У Canvas-у идите на **Admin** > изаберите свој налог > **Settings** > таб **Apps**.

Кликните **+ App** и конфигуришите:

1. Подесите **Configuration Type** на **By Client ID**.
2. Налепите **Client ID** из табеле Developer Keys.
3. Кликните **Submit**.
4. Потврдите инсталацију када буде затражено.

FastComments је сада инсталиран на нивоу налога и доступан свим курсевима.