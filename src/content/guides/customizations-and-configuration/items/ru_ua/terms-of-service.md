FastComments позволяет требовать от пользователей, комментирующих впервые, принятия вашего Пользовательского соглашения перед отправкой комментария.

When enabled:
- **Anonymous users** will see a TOS checkbox every time they comment
- **Authenticated users** will see the checkbox only on their first comment, or when you update your TOS

### Конфигурация

Перейдите на страницу настройки виджета и включите флажок "Require Terms of Service acceptance". После включения вы увидите следующие параметры:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; title='Terms of Service Options' app-screenshot-end]

- **TOS Text Mode**: По умолчанию чекбокс отображает "I agree to the Terms of Service and Privacy Policy" со ссылками на оба документа. Выберите "Customize text per locale", чтобы указать собственный текст для каждого языка.
- **TOS Last Updated Date**: Когда вы обновляете своё Пользовательское соглашение, установите эту дату. Пользователи, принявшие соглашение до этой даты, будут вынуждены принять его снова.

### Как это работает

- Отметка времени принятия Пользовательского соглашения сохраняется для каждого пользователя и каждого комментария
- Когда пользователь принимает Пользовательское соглашение, дата фиксируется в его профиле пользователя (по аренде/тенанту)
- Если вы устанавливаете дату "Last Updated", которая позже даты принятия пользователем, ему потребуется принять соглашение снова
- Для анонимных пользователей, которых нельзя отслеживать, чекбокс отображается при каждой отправке комментария