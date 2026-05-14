Эта страница описывает добавление FastComments в курс Brightspace после того, как администратор зарегистрировал инструмент и создал deployment. Если инструмент ещё не зарегистрирован, сначала ознакомьтесь с руководством по регистрации D2L.

Brightspace предоставляет два варианта создания содержимого: **Classic Content** и **New Content Experience** (также называемый **Lessons**). Оба варианта поддерживают FastComments, но пути в меню различаются. В каждом разделе ниже описаны оба варианта там, где они расходятся.

#### Locate the FastComments Tool

Инструмент FastComments отображается в двух местах внутри редактора содержания курса:

1. Activity picker, доступный через кнопку **Add Existing** модуля/юнита (в старых версиях Brightspace помечена как **Add Existing Activities**). FastComments отображается прямо в picker в современных сборках Brightspace; в старых версиях он вложен в подменю **External Learning Tools**. Любой из путей добавляет FastComments как отдельную тему.
2. Диалог **Insert Stuff** внутри HTML-редактора, в разделе **LTI Advantage**. Это позволяет встроить FastComments внутри HTML-темы через поток LTI deep linking.

Если FastComments не отображается ни в одном из picker'ов, значит deployment не включён для org unit, содержащей курс. Попросите администратора Brightspace открыть **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, открыть deployment и добавить org unit курса (или родительский org unit) в список **Org Units**.

#### Add FastComments as a Topic in a Module

Classic Content:

1. Откройте курс и нажмите **Content** в navbar.
2. Выберите модуль, который должен содержать обсуждение (или создайте его через **Add a module**).
3. Нажмите **Add Existing** (старый Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. В picker'е нажмите **FastComments**. Brightspace создаст тему в модуле и вернёт вас в просмотр содержания.
5. Откройте новую тему. Переименуйте её в что-то описательное, например `FastComments Discussion`, используя встроенный редактор заголовка.

New Content Experience (Lessons):

1. Откройте курс и нажмите **Content**.
2. Откройте unit и lesson, которые должны содержать обсуждение.
3. Нажмите **Add** > **Existing Activity** и выберите **FastComments** (в старых версиях Brightspace: вложено в **External Learning Tools**).
4. Активность добавится в lesson.
5. Нажмите на заголовок активности, чтобы переименовать её.

При первом открытии темы любым пользователем (инструктором или студентом) FastComments инициализирует thread для этой resource link. Тред привязан к resource link ID, поэтому переименование или перемещение темы не меняет загружаемый тред.

#### Embed FastComments Inline in an HTML Topic

Используйте этот поток, когда вы хотите, чтобы комментарии отображались под чтением, видео или другим содержимым внутри той же страницы темы, а не как отдельная тема.

1. Откройте или создайте HTML-тему в модуле/lesson.
2. Нажмите **Edit HTML**, чтобы открыть HTML-редактор Brightspace.
3. Поместите курсор туда, где должен появиться комментарийный тред.
4. Нажмите кнопку **Insert Stuff** (иконка пазла в панели редактора).
5. В диалоге Insert Stuff прокрутите до **LTI Advantage** и нажмите **FastComments**.
6. FastComments откроет deep linking picker. Подтвердите размещение (параметры по умолчанию подходят для дискуссий по содержимому); нажмите **Insert** или **Continue**.
7. Brightspace вернётся в HTML-редактор с блоком-заполнителем, представляющим LTI launch. Нажмите **Save and Close** в теме.

Когда тема загрузится, Brightspace заменит заполнитель iframe, который автоматически запустит FastComments через LTI. Студенты увидят тред обсуждения встраиваемым на странице.

Одна HTML-тема может содержать несколько deep-linked встраиваний FastComments. Каждое встраивание получает собственный тред, потому что каждая deep link создаёт отдельный resource link ID.

#### Module Topic vs Inline Quicklink

Выберите подход с **module topic**, когда:

- Обсуждение является основной активностью на этом шаге модуля.
- Вы хотите, чтобы тема отображалась в оглавлении Brightspace, отслеживании выполнения и Class Progress.

Выберите подход **inline embed**, когда:

- Комментарии должны располагаться под другим содержимым на той же странице.
- Вы не хотите отдельного отслеживаемого в оглавлении элемента с отслеживанием выполнения.

#### Visibility, Draft, and Release Conditions

Новая тема FastComments по умолчанию видна студентам. Чтобы скрыть её пока вы настраиваете:

1. В редакторе содержания нажмите заголовок темы (Classic) или меню с тремя точками на активности (New Content Experience).
2. Установите статус в **Draft** (Classic) или переключите **Visibility** в выкл. (New Content Experience).

Draft-темы невидимы студентам. Инструкторы и TAs по-прежнему видят их с бейджем «Draft».

Чтобы ограничить тему для определённой группы или секции:

1. Откройте тему.
2. Нажмите меню заголовка темы > **Edit Properties In-place** (Classic) или **Edit** > **Restrictions** (New Content Experience).
3. В разделе **Release Conditions** нажмите **Create**.
4. Выберите **Group enrollment** или **Section enrollment**, выберите группу/секцию и сохраните.

Условия публикации сочетаются с собственной ролью маппинга FastComments. Студенты, которые не видят тему, не получают LTI launch.

#### What Students See on First Launch

Когда студент нажимает тему (или загружает HTML-тему с встраиванием):

1. Brightspace выполняет LTI 1.3 launch в фоне.
2. FastComments получает имя студента, email, URL аватара и роль LMS, и автоматически выполняет вход. Запроса на вход в FastComments не появляется.
3. Тред комментариев для этого resource link отображается внутри iframe Brightspace.

Соответствие ролей при запуске:

- Brightspace `Administrator` становится в FastComments администратором треда (полный модерационный доступ, удаление, бан и доступ к конфигурации).
- Brightspace `Instructor` становится в FastComments модератором (pin, hide, delete, ban).
- Все остальные роли (`Learner`, `TeachingAssistant`, и т.д.) становятся обычными комментаторами.

Комментарии привязываются к аккаунту студента в Brightspace. Если студент изменит своё имя или аватар в Brightspace, следующее LTI-запуск синхронизирует изменения.

#### Iframe Height and Resize

FastComments посылает `org.imsglobal.lti.frameResize` postMessage при каждом рендере треда и при изменениях содержимого (новый комментарий, раскрытие ответов). Brightspace слушает это сообщение и подстраивает высоту iframe, чтобы тред не обрезался и не появлялась внутренняя полоса прокрутки.

Если iframe остаётся фиксированной маленькой высоты:

- Убедитесь, что курс загружается по HTTPS. Слушатель postMessage Brightspace отклоняет фреймы смешанного контента.
- Убедитесь, что никакое расширение браузера не блокирует канал postMessage.
- Для inline-встраиваний в HTML-теме окружающий HTML не должен оборачивать iframe в контейнер с фиксированной высотой. Удалите любой inline `style="height: ..."` у родительского элемента.

#### Brightspace-Specific Gotchas

**Tool not showing in the Add Existing picker.** Deployment не включён для org unit этого курса. Администратор должен добавить org unit (или родительский) в список Org Units деплоя. Регистрация инструмента сама по себе недостаточна; deployment определяет, какие курсы видят инструмент.

**`deployment_id` mismatch on launch.** FastComments TOFU-привязывает первый `deployment_id`, который он видит при регистрации. Если администратор удаляет исходный deployment и создаёт новый, запуски из нового deployment отклоняются с ошибкой несоответствия deployment. Исправление — повторно зарегистрировать FastComments (сгенерировать новый registration URL и снова выполнить Dynamic Registration); старая запись конфигурации будет заменена.

**Tool launches but shows "Invalid LTI launch".** Курс находится в другом tenant/структуре org, чем покрывает deployment, либо deployment был отключён после регистрации. Проверьте ещё раз **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > переключатель **Enabled** и список org units в deployment.

**Names and roles missing inside FastComments.** Brightspace отправляет LTI-запуски с Names and Role Provisioning Services (NRPS) claims. Если курс был обновлён из старой LTI 1.1 ссылки, в запуске отсутствуют claims `name` и `email`. Повторно добавьте тему FastComments через **Add Existing** (не мигрируйте старую ссылку), чтобы запуск использовал LTI 1.3.

**Embed shows a login screen instead of auto-SSO.** HTML-тема была вставлена как простой `<iframe>`, указывающий на FastComments, а не через **Insert Stuff** > **LTI Advantage**. Простые iframes пропускают LTI launch и попадают на публичную страницу FastComments. Удалите iframe и вставьте заново через поток Insert Stuff.