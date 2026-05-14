Moodle 4.0+ поддерживает динамическую регистрацию LTI 1.3 через плагин External Tool.

#### Откройте экран управления инструментами

1. Войдите в Moodle под учетной записью администратора сайта.
2. Перейдите в **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools**.

#### Вставьте URL

Вы увидите карточку с меткой **Tool URL**. Вставьте URL регистрации FastComments в текстовое поле и нажмите **Add LTI Advantage**.

Moodle откроет экран регистрации, показывающий идентификацию инструмента и запрашиваемые разрешения. Проверьте и нажмите **Activate** (или **Register**, в зависимости от версии Moodle).

Всплывающее окно закроется после завершения регистрации; новый инструмент FastComments появится в списке **Tools** со статусом **Active**.

#### Сделайте его доступным

По умолчанию Moodle добавляет новые инструменты в список «Course tools», но не показывает их в выборе активности. Чтобы сделать FastComments доступным на уровне курса:

1. Нажмите на значок шестеренки на плитке FastComments.
2. В разделе **Tool configuration usage** выберите **Show in activity chooser and as a preconfigured tool**.
3. Сохраните.

Инструкторы теперь могут добавить FastComments в любой курс через **Add an activity or resource** > **FastComments**.