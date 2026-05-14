После того, как FastComments зарегистрирован в платформе, преподаватели добавляют его в содержимое курса с помощью стандартных потоков добавления внешних инструментов платформы. На этой странице описаны Sakai 23.x и Schoology Enterprise.

#### Sakai

**1. Add FastComments to a site**

Ответственный за сайт включает инструмент для каждого сайта отдельно:

1. Откройте сайт и нажмите **Site Info** в левой навигации.
2. Нажмите **Manage Tools**.
3. Прокрутите до списка **External Tools** и включите **FastComments**.
4. Нажмите **Continue**, проверьте список инструментов, затем нажмите **Finish**.

FastComments теперь появляется как элемент левой навигации на сайте.

**2. Reorder the left-nav entry**

Перейдите в **Site Info** > **Tool Order**. Перетащите **FastComments** в нужное положение и нажмите **Save**. С этого экрана также можно переименовать метку навигации и скрыть её от студентов.

**3. Embed inline in a Lessons page**

Чтобы разместить FastComments непосредственно внутри страницы **Lessons**, а не как отдельный инструмент в левой навигации:

1. Откройте инструмент **Lessons** на сайте.
2. Нажмите **Add Content** > **Add External Tool**.
3. Выберите **FastComments** из списка.
4. Если FastComments рекламировал Deep Linking при регистрации, Sakai открывает селектор содержимого инструмента, чтобы вы могли выбрать или пометить тему. Если Deep Linking не был рекламирован, Sakai вставляет ссылку запуска по умолчанию.
5. Сохраните элемент Lessons.

Каждый встроенный экземпляр получает собственную ветку комментариев, привязанную к этой ссылке ресурса.

**4. Permission tweaks for student access**

Sakai контролирует запуски внешних инструментов через Realms. Чтобы убедиться, что студенты могут запускать FastComments:

1. Войдите как администратор Sakai и откройте **Administration Workspace** > **Realms**.
2. Откройте соответствующий realm (например, `!site.template.course` или конкретный realm сайта).
3. Убедитесь, что роль `access` имеет включённый `lti.launch` и что разрешения ролей в группе **external.tools** предоставлены.
4. Сохраните realm.

Для переопределений на уровне сайта ответственный может изменить видимость инструмента по ролям на экране **Site Info** > **Tool Order**, скрывая или показывая FastComments для каждой роли.

**5. What students see**

Студенты нажимают элемент FastComments в левой навигации (или прокручивают до встроенного блока Lessons) и попадают непосредственно в представление с веткой комментариев. SSO выполняется автоматически: Sakai отправляет учётные данные пользователя в LTI-запуске, и FastComments автоматически выполняет вход под их аккаунтом Sakai.

Сопоставление ролей:

- Sakai `Instructor` -> FastComments moderator
- Sakai `Admin` (admin in Administration Workspace) -> FastComments admin
- Sakai `Student` / `access` -> FastComments commenter

**6. Sakai gotchas**

- **Tool not visible in Manage Tools.** Если FastComments не отображается в списке External Tools, администратору Sakai нужно открыть реестр инструментов (**Administration Workspace** > **External Tools** > **FastComments**) и установить **Stealthed** в `false`. Инструменты со скрытым режимом (stealthed) не видны в селекторе Manage Tools для отдельных сайтов.
- **Launches breaking in shared-session browsers.** CSRF-токен портала Sakai привязан к сессии браузера. Если студент вошёл в два сайта Sakai в разных вкладках или у него есть устаревшая сессия, запуск возвращает 403. Решение: закройте другие вкладки Sakai, выйдите из системы, войдите снова и запустите заново. Администраторы также могут увеличить `sakai.csrf.token.cache.ttl`, если это происходит по всему кластеру.
- **Frame embedding.** Убедитесь, что `lti.frameheight` в `sakai.properties` достаточно велик (600 или больше), чтобы ветка комментариев не обрезалась внутри страницы Lessons.

#### Schoology

Schoology Enterprise имеет две сценария установки. Уточните, какой из них применим, прежде чем добавлять инструмент в курс.

**1. Two installation scenarios**

- **(a) Enterprise-level install.** Системный администратор Schoology установил FastComments на уровне организации и назначил его всем курсам или определённым шаблонам курсов. Преподавателям не нужно устанавливать инструмент — они переходят сразу к разделу "Add Materials".
- **(b) Instructor self-install.** Преподаватель устанавливает инструмент в отдельный курс через **Course Options** > **External Tools** > **Install LTI Apps**. Самостоятельная установка требует предварительного одобрения приложения FastComments системным администратором на уровне организации.

**2. Add FastComments as a course material**

Внутри курса:

1. Откройте курс и перейдите в **Materials**.
2. Нажмите **Add Materials** > **Add File/Link/External Tool**.
3. Выберите **External Tool**.
4. Выберите **FastComments** из списка зарегистрированных инструментов.
5. Установите **Name** (именно это видят студенты в списке материалов) и при желании добавьте **Description**.
6. Оставьте **Enable Grading** (передача оценок) **OFF**. FastComments не отправляет оценки обратно в Schoology, поэтому включение передачи оценок создаёт пустой столбец в журнале оценок.
7. Нажмите **Submit**.

Материал теперь появляется в списке материалов курса и открывает ветку FastComments при нажатии.

**3. Inline embedding via the Rich Text editor**

Если системный администратор включил Deep Linking placement для FastComments при регистрации, преподаватели могут встроить ветку комментариев в любое поле Rich Text (инструкции к заданию, содержимое страницы, подсказки к обсуждению):

1. Откройте Rich Text editor на нужной странице.
2. Нажмите значок **External Tool** (значок пазла) на панели инструментов.
3. Выберите **FastComments**.
4. Настройте встраивание в диалоге deep-linking и нажмите **Insert**.
5. Сохраните страницу.

Если кнопка External Tool не отображается в Rich Text editor, Deep Linking отключён для этого инструмента в данном тенанте. См. раздел с проблемами ниже.

**4. Visibility and section assignments**

Schoology ограничивает доступность инструментов по секциям через Course Options:

1. В курсе нажмите **Course Options** > **External Tools**.
2. Для каждого установленного LTI-приложения вы контролируете, доступно ли оно всем секциям курса или только определённым секциям.
3. Чтобы ограничить FastComments для определённых секций, снимите флажки с секций, которым не следует видеть инструмент.
4. Доступ на уровне секции также определяет, какие секции видят пункт **Add Materials** > **External Tool** для FastComments.

**5. What students see**

Студенты нажимают материал FastComments (или прокручивают до встроенного фрагмента) и попадают в ветку обсуждения. SSO выполняется автоматически через Schoology LTI-запуск под их аккаунтом Schoology.

Сопоставление ролей:

- Schoology `Administrator` -> FastComments admin
- Schoology `Instructor` -> FastComments moderator
- Schoology `Student` -> FastComments commenter

**6. Schoology gotchas**

- **Enterprise-only.** Личные и бесплатные аккаунты Schoology не могут устанавливать инструменты LTI 1.3. Если ваш тенант находится на бесплатном тарифе, опция **External Tools** отсутствует в Course Options. Обновитесь до Schoology Enterprise, чтобы использовать FastComments.
- **Deep Linking disabled by tenant default.** Некоторые тенанты Schoology ограничивают размещение Deep Linking на уровне организации. В этом случае преподаватели видят только поток **Add Materials** > **External Tool**, а не кнопку External Tool в Rich Text editor. Чтобы включить встроенное встраивание, системный администратор должен перейти в **System Settings** > **Integration** > **LTI 1.3** > **FastComments** и включить размещение **Content Item / Deep Linking**, затем сохранить.
- **Per-section assignment override.** Если FastComments назначен на уровне организации, но преподаватель не видит его в **Add Materials**, секция курса исключена в назначении на уровне организации. Попросите системного администратора добавить секцию в назначение приложения FastComments.
- **Material name vs. thread identity.** Переименование материала в Schoology не перемещает ветку комментариев. Ветки привязаны к LTI resource link ID, поэтому переименование сохраняет ту же ветку; удаление и повторное создание материала создаёт новую пустую ветку.