Moodle 4.0+ поддерживает LTI 1.3 Dynamic Registration через плагин External tool.

#### Open the Tool Management Screen

1. Войдите в Moodle под учётной записью администратора сайта.
2. Перейдите в **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools**.

#### Paste the URL

Вы увидите карточку с меткой **Tool URL**. Вставьте регистрационный URL FastComments в поле ввода и нажмите **Add LTI Advantage**.

Moodle откроет экран регистрации, показывающий идентичность инструмента и запрашиваемые разрешения. Проверьте и нажмите **Activate** (или **Register**, в зависимости от версии Moodle).

Всплывающее окно закроется после завершения регистрации; новый инструмент FastComments появится в списке **Tools** со статусом **Active**.

#### Make It Available

По умолчанию Moodle добавляет новые инструменты в список "Course tools", но не отображает их в списке выбора активности. Чтобы сделать FastComments доступным для всего курса:

1. Нажмите значок шестерёнки на плитке FastComments.
2. В разделе **Tool configuration usage** выберите **Show in activity chooser and as a preconfigured tool**.
3. Сохраните.

Преподаватели теперь могут добавить FastComments в любой курс через **Add an activity or resource** > **FastComments**.