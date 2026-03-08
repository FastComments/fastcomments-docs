---
1. Идите на <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">вашу FastComments LTI конфигурацију</a>.
2. Унесите **Configuration Name** и ваш **Platform URL** (нпр. `https://yourschool.instructure.com`). Изаберите која **Placements** да омогућите (Assignment View и/или Editor Button — оба су подразумевано укључена). Кликните **Create Configuration**. Чаробњак прелази на корак 2 и приказује ваш **Configuration URL**.
3. У Canvas-у идите на **Admin > Developer Keys > + Developer Key > LTI Key**. Поставите **Method** на "Enter URL" и налепите ваш **Configuration URL**. Сачувајте кључ, затим његово **State** поставите на **ON** и кликните **Allow** када будете упитани.
4. Копирајте број **Client ID** из табеле Developer Keys у Canvas-у. Вратите се у FastComments, налепите га у поље **Client ID** и кликните **Save & Continue**.
5. Прегледајте сажетак конфигурације и кликните **Enable Integration** да бисте активирали интеграцију.
6. Инсталирајте External App у Canvas-у (**Admin > Settings > Apps > + App > By Client ID**). Коментари ће се аутоматски појавити испод задатака, а наставници могу уградити FastComments на странице, квизове и обавештења преко дугмета на траци алата Rich Content Editor.

---