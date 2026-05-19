**Корисите Moodle?** Такође објављујемо намјенски Moodle додатак за FastComments са ближом интеграцијом него LTI 1.3 (хукови за синхронизацију оцјена, дublje извештавање о активностима, нативни Moodle интерфејс за подешавања). Погледајте <a href="/guide-installation-moodle.html" target="_blank">упутство за инсталацију Moodle додатка</a>. Ток LTI 1.3 испод је прави избор ако желите једну регистрацију која покрива и друге LMS-ове, или ако ваш Moodle администратор неће инсталирати додатке трећих страна.

Moodle 4.0+ подржава LTI 1.3 Dynamic Registration преко додатка External Tool.

#### Отворите екран за управљање алатком

1. Пријавите се у Moodle као администратор сајта.
2. Идите на **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools**.

#### Залепите URL

Видећете картицу означену као **Tool URL**. Залепите FastComments URL за регистрацију (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">добијте га овде</a>) у текстуално поље и кликните **Add LTI Advantage**.

Moodle отвара екран за регистрацију који показује идентитет алатке и дозволе које тражи. Прегледајте и кликните **Activate** (или **Register**, у зависности од верзије Moodle-а).

Поп-уп се затвара када се регистрација заврши; нова FastComments алатка ће се појавити на листи **Tools** са статусом **Active**.

#### Омогућите приступ

По подразумеваној поставци Moodle додаје нове алатке на листу "Course tools" али их не приказује у activity picker-у. Да бисте FastComments учинили доступним ученицима широм курса:

1. Кликните икону зупчаника на FastComments картици.
2. Под **Tool configuration usage**, изаберите **Show in activity chooser and as a preconfigured tool**.
3. Сачувајте.

Наставници сада могу додати FastComments у било који курс преко **Add an activity or resource** > **FastComments**.