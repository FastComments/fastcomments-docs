#### Navigate to Canvas LTI Config

Пријавите се на ваш FastComments налог и идите на <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">My Account > Canvas LTI Config</a>.

#### Create a New LTI Configuration

Означите поље за потврду **Enable LTI**. Пojaviће се поља за конфигурацију:

- **Configuration Name** - опционална ознака за идентификацију ове конфигурације (корисно ако повежете више Canvas инстанци).
- **Platform URL** - URL ваше Canvas инстанце (нпр. `https://yourschool.instructure.com`). Можете оставити ово празно за сада и попунити га након креирања Developer Key.
- **Client ID** - оставите ово празно за сада. Попунићете га након креирања Developer Key у Canvas.
- **Deployment ID** - оставите ово празно за сада.
- **Comment Style** - одаберите између Comments, Collab Chat, или Both. Погледајте [Commenting Styles](#canvas-lms-commenting-styles) за детаље.

Кликните на **Add** да бисте креирали конфигурацију.

#### Copy the Configuration URLs

Након чувања, појавиће се три URL-а:

- **Configuration URL** - налепићете ово у Canvas када креирате Developer Key.
- **OIDC Login URL** - који Canvas користи за LTI процес пријављивања (аутоматски конфигурисан преко Configuration URL).
- **Launch URL** - крајња тачка коју Canvas позива када студент отвори FastComments (аутоматски конфигурисана преко Configuration URL).

Копирајте **Configuration URL**. Биће вам потребан у следећем кораку.