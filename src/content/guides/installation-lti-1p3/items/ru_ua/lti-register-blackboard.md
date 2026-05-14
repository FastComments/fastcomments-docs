Blackboard Learn SaaS и Ultra поддерживают динамическую регистрацию LTI 1.3.

#### Откройте экран поставщика инструментов

1. Войдите в Blackboard как системный администратор.
2. Перейдите в **Administrator Panel** > **Integrations** > **LTI Tool Providers**.
3. Нажмите **Register LTI 1.3 / LTI Advantage Tool**.

Если вы видите только "Register LTI 1.1 Provider", ваша версия Blackboard ещё не поддерживает LTI 1.3 — обновите её или обратитесь в службу поддержки Blackboard.

#### Вставьте URL

Вставьте URL регистрации FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">получить его здесь</a>) в поле **Client ID** / **Registration URL** (наименование полей в Blackboard зависит от версии). Отправьте.

Blackboard выполняет процесс обмена регистрационными данными с FastComments и показывает экран подтверждения.

#### Одобрите и включите

Blackboard по умолчанию помечает вновь зарегистрированные инструменты как **Approved but excluded**:

1. Найдите запись FastComments в списке поставщиков инструментов.
2. Откройте меню и выберите **Edit**.
3. Установите **Tool Status** в значение **Approved**.
4. В разделе **Institution Policies** проверьте, какие данные пользователя отправляются (name, email, role). Сохраните.

Инструмент теперь доступен преподавателям при добавлении контента в курсы.