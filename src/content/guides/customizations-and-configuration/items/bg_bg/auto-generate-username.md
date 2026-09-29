When users comment or vote, and they are not logged in, they will be asked to provide their email and username.

For some sites, asking a visitor to invent a unique username is a hurdle, particularly on mobile. FastComments can generate a neutral username for each new visitor and prefill it in the username field, like `BraveOtter4172`.

The visitor can leave it as-is, or replace it with a name of their choosing.

This can be enabled from the Customization UI, under the setting called `Generate Usernames Automatically`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.auto-generate-username'; alt='Опцията „Generate Usernames Automatically“ в потребителския интерфейс за персонализиране на уиджета'; title='Автоматично генериране на потребителски имена' app-screenshot-end]

#### Как се държи

- Всяко генерирано име е уникално. То се проверява спрямо съществуващите акаунти и се запазва за сесията на браузъра на този посетител, така че двама посетители да не получат едно и също име.
- Името се генерира само за посетители, които все още нямат такова. Вписаните потребители, потребители чрез SSO и посетители, които вече са коментирали, запазват съществуващото си име.
- Работи както с, така и без [анонимно коментиране](/guide-customizations-and-configuration.html#allow-anon). Когато анонимното коментиране е изключено, посетителят все още въвежда имейла си, но вече не трябва да мисли за потребителско име.
- Връщащ се посетител, който въведе имейл, който е използвал преди, се съпоставя със съществуващия си акаунт и запазва името от този акаунт.
- Ако е зададено и [по подразбиране потребителско име](/guide-customizations-and-configuration.html#default-username), генерираното име има предимство.

---