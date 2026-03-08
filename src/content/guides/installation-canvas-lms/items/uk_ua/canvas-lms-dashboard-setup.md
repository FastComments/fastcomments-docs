#### Navigate to Canvas LTI Config

Увійдіть у свій обліковий запис FastComments і перейдіть до <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">My Account > Canvas LTI Config</a>.

#### Create a New LTI Configuration

Поставте прапорець **Enable LTI**. З’являться поля конфігурації:

- **Configuration Name** - необов’язкова мітка для ідентифікації цієї конфігурації (корисно, якщо ви підключаєте кілька екземплярів Canvas).
- **Platform URL** - URL вашого екземпляру Canvas (наприклад `https://yourschool.instructure.com`). Ви можете залишити це поле порожнім зараз і заповнити його після створення Developer Key.
- **Client ID** - залиште це поле порожнім зараз. Ви заповните його після створення Developer Key у Canvas.
- **Deployment ID** - залиште це поле порожнім зараз.
- **Comment Style** - виберіть між Comments, Collab Chat або Both. Див. [Commenting Styles](#canvas-lms-commenting-styles) для деталей.

Натисніть **Add**, щоб створити конфігурацію.

#### Copy the Configuration URLs

Після збереження з’являться три URL:

- **Configuration URL** - ви вставите це в Canvas під час створення Developer Key.
- **OIDC Login URL** - використовується Canvas для LTI-потоку входу (автоматично налаштовується через Configuration URL).
- **Launch URL** - кінцева точка, яку викликає Canvas, коли студент відкриває FastComments (автоматично налаштовується через Configuration URL).

Скопіюйте **Configuration URL**. Він знадобиться на наступному кроці.