#### Навигирайте до Canvas LTI Config

Влезте в акаунта си в FastComments и отидете на <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">My Account > Canvas LTI Config</a>.

#### Create a New LTI Configuration

Поставете отметка в полето **Enable LTI**. Ще се появят полетата за конфигурация:

- **Configuration Name** - по избор етикет за идентифициране на тази конфигурация (полезно, ако свържете няколко инстанции на Canvas).
- **Platform URL** - URL на вашата Canvas инстанция (например `https://yourschool.instructure.com`). Можете да го оставите празно за момента и да го попълните след създаването на Developer Key.
- **Client ID** - оставете това празно за момента. Ще го попълните след създаването на Developer Key в Canvas.
- **Deployment ID** - оставете това празно за момента.
- **Comment Style** - изберете между Comments, Collab Chat или Both. Вижте [Commenting Styles](#canvas-lms-commenting-styles) за подробности.

Щракнете върху **Add**, за да създадете конфигурацията.

#### Copy the Configuration URLs

След записване ще се появят три URL адреса:

- **Configuration URL** - ще го поставите в Canvas при създаването на Developer Key.
- **OIDC Login URL** - използва се от Canvas за LTI процеса на вход (автоматично конфигурирано чрез Configuration URL).
- **Launch URL** - крайна точка, към която Canvas прави повикване когато студент отвори FastComments (автоматично конфигурирано чрез Configuration URL).

Копирайте **Configuration URL**. Ще ви трябва в следващата стъпка.