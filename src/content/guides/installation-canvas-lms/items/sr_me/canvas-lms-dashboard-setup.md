#### Navigate to Canvas LTI Config

Пријавите се на свој FastComments налог и идите на <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">Мој налог > Canvas LTI конфигурација</a>.

#### Create a New LTI Configuration

Означите поље за потврду **Омогући LTI**. Појавиће се поља за конфигурацију:

- **Назив конфигурације** - опционална ознака за идентификацију ове конфигурације (корисно ако повежете више Canvas инстанци).
- **URL платформе** - URL ваше Canvas инстанце (нпр. `https://yourschool.instructure.com`). Можете ово оставити празно за сада и попунити након креирања Developer Key-а.
- **Client ID** - оставите ово празно за сада. Попунићете га након креирања Developer Key-а у Canvas-у.
- **Deployment ID** - оставите ово празно за сада.
- **Стил коментара** - изаберите између Comments, Collab Chat, или Both. Погледајте [Стилови коментарисања](#canvas-lms-commenting-styles) за детаље.

Кликните **Додај** да бисте креирали конфигурацију.

#### Copy the Configuration URLs

Након сачувања, појавиће се три URL-а:

- **URL конфигурације** - налепићете ово у Canvas приликом креирања Developer Key-а.
- **OIDC Login URL** - које Canvas користи за LTI ток пријављивања (аутоматски конфигурисано преко URL-а конфигурације).
- **Launch URL** - крајња тачка коју Canvas позива када студент отвори FastComments (аутоматски конфигурисано преко URL-а конфигурације).

Копирајте **URL конфигурације**. Биће вам потребан у следећем кораку.