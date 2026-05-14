Blackboard Learn SaaS и Ultra поддерживают динамическую регистрацию LTI 1.3.

#### Откройте экран поставщика инструментов

1. Войдите в Blackboard как системный администратор.
2. Перейдите в **Administrator Panel** > **Integrations** > **LTI Tool Providers**.
3. Нажмите **Register LTI 1.3 / LTI Advantage Tool**.

Если вы видите только "Register LTI 1.1 Provider", ваша версия Blackboard ещё не поддерживает LTI 1.3 — обновите систему или обратитесь в службу поддержки Blackboard.

#### Вставьте URL

Вставьте URL регистрации FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">получить её здесь</a>) в поле **Client ID** / **Registration URL** (названия полей в Blackboard различаются в зависимости от версии). Нажмите Submit.

Blackboard выполняет обмен регистрационными данными с FastComments и показывает экран подтверждения.

#### Одобрите и включите

Blackboard помечает вновь зарегистрированные инструменты как **Approved but excluded** по умолчанию:

1. Найдите запись FastComments в списке поставщиков инструментов.
2. Откройте меню и выберите **Edit**.
3. Установите для **Tool Status** значение **Approved**.
4. В разделе **Institution Policies** проверьте, какие данные пользователя отправляются (имя, электронная почта, роль). Сохраните.

Инструмент теперь доступен преподавателям при добавлении ими контента в курсы.