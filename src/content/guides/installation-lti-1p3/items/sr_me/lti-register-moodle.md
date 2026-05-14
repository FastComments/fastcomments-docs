**Користите Moodle?** Такође објављујемо посебан Moodle додатак за FastComments са чвршћом интеграцијом него LTI 1.3 (hook-ови за синхронизацију оцена, дубље извештавање активности, нативни кориснички интерфејс за подешавања у Moodle-у). Погледајте <a href="/guide-installation-moodle.html" target="_blank">водич за инсталацију Moodle додатка</a>. LTI 1.3 ток приказан испод је прави избор ако желите једну регистрацију која покрива и друге LMS-ове, или ако ваш Moodle администратор неће инсталирати додатке трећих страна.

Moodle 4.0+ подржава LTI 1.3 Dynamic Registration преко додатка External Tool.

#### Отворите екран за управљање алаткама

1. Пријавите се у Moodle као администратор сајта.
2. Идите на **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools**.

#### Налепите URL

Видећете картицу означену **Tool URL**. Налепите FastComments registration URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">добијте га овде</a>) у поље за текст и кликните **Add LTI Advantage**.

Moodle отвори екран за регистрацију који приказује идентитет алата и дозволе које тражи. Прегледајте и кликните **Activate** (или **Register**, у зависности од верзије Moodle-а).

Поп-ап се затвара када се регистрација заврши; нови FastComments алат се појављује у листи **Tools** са статусом **Active**.

#### Учините га доступним

По подразумевaњу Moodle додаје нове алатке у листу "Course tools" али их не приказује у изборнику активности. Да бисте омогућили FastComments широм курса:

1. Кликните икону зупчаника на FastComments плочици.
2. Под **Tool configuration usage**, одаберите **Show in activity chooser and as a preconfigured tool**.
3. Сачувајте.

Наставници сада могу додати FastComments у било који курс преко **Add an activity or resource** > **FastComments**.