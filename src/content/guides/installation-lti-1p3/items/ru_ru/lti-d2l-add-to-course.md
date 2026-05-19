Эта страница описывает добавление FastComments в курс Brightspace после того, как администратор зарегистрировал инструмент и создал deployment. Если инструмент ещё не зарегистрирован, сначала смотрите руководство по регистрации D2L.

<div class="screenshot white-bg">
    <div class="title">FastComments embedded as a unit topic in Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments, запущенный внутри Brightspace unit, показывающий древовидные комментарии и @-mention picker" />
</div>

Brightspace поставляется с двумя способами создания контента: **Classic Content** и **New Content Experience** (также называемая **Lessons**). Оба поддерживают FastComments, но пути в меню различаются. Каждая секция ниже охватывает оба варианта там, где они расходятся.

#### Locate the FastComments Tool

Инструмент FastComments появляется в двух местах внутри редактора содержимого курса:

1. В activity picker, доступном через кнопку **Add Existing** модуля/юнита (в старых версиях Brightspace помечена как **Add Existing Activities**). В текущих сборках FastComments отображается непосредственно в picker; в старых версиях он находится в подменю **External Learning Tools**. Любой из путей добавляет FastComments как отдельную тему.
2. В диалоге **Insert Stuff** внутри HTML-редактора, в разделе **LTI Advantage**. Это встраивает FastComments внутри HTML-темы через LTI deep linking flow.

Если FastComments не появляется ни в одном picker, deployment не включён для org unit, в котором находится курс. Попросите администратора Brightspace открыть **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, открыть deployment и добавить org unit курса (или родительский org unit) в раздел **Org Units**.

#### Add FastComments as a Topic in a Module

Classic Content:

1. Откройте курс и нажмите **Content** в навигационной панели.
2. Выберите модуль, который должен содержать обсуждение (или создайте его через **Add a module**).
3. Нажмите **Add Existing** (в старых версиях Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. В picker нажмите **FastComments**. Brightspace создаст тему в модуле и вернёт вас в просмотр содержимого.
5. Нажмите новую тему. Переименуйте её в что-то описательное, например `FastComments Discussion` с помощью inline title editor.

New Content Experience (Lessons):

1. Откройте курс и нажмите **Content**.
2. Откройте unit и lesson, которые должны содержать обсуждение.
3. Нажмите **Add** > **Existing Activity** и выберите **FastComments** (в старых версиях Brightspace: вложено в **External Learning Tools**).
4. Активность будет добавлена в lesson.
5. Нажмите название активности, чтобы переименовать её.

Впервые, когда любой пользователь (инструктор или студент) откроет тему, FastComments инициализирует thread для этой resource link. Thread привязан к resource link ID, поэтому переименование или перемещение темы не меняет загружаемый thread.

#### Embed FastComments Inline in an HTML Topic

Используйте этот поток, когда вы хотите, чтобы комментарии отображались под текстом, видео или другим содержимым внутри той же страницы темы, а не как отдельная тема.

1. Откройте или создайте HTML-тему в модуле/lesson.
2. Нажмите **Edit HTML**, чтобы открыть HTML-редактор Brightspace.
3. Поместите курсор в то место, где должен появиться поток комментариев.
4. Нажмите кнопку **Insert Stuff** (иконка паззла в панели редактора).
5. В диалоге Insert Stuff пролистайте до **LTI Advantage** и нажмите **FastComments**.
6. FastComments откроет deep linking picker. Подтвердите размещение (опции по умолчанию подходят для обсуждений контента); нажмите **Insert** или **Continue**.
7. Brightspace вернётся в HTML-редактор с плейсхолдер-блоком, представляющим LTI launch. Нажмите **Save and Close** в теме.

Когда тема загружается, Brightspace заменяет плейсхолдер на iframe, который автоматически запускает FastComments через LTI. Студенты увидят поток обсуждения встроенным на странице.

Одна HTML-тема может содержать несколько deep-linked FastComments embed'ов. Каждый embed получает свой собственный thread, потому что каждая deep link создаёт уникальный resource link ID.

#### Module Topic vs Inline Quicklink

Выбирайте подход с **module topic**, когда:

- Обсуждение является основной активностью на этом шаге модуля.
- Вы хотите, чтобы тема отображалась в оглавлении Brightspace, в отслеживании выполнения и в Class Progress.

Выбирайте подход с **inline embed**, когда:

- Комментарии должны располагаться под другим контентом на той же странице.
- Вы не хотите отдельный элемент, отслеживаемый для выполнения, в оглавлении.

#### Visibility, Draft, and Release Conditions

Новая тема FastComments по умолчанию видна студентам. Чтобы скрыть её пока вы настраиваете:

1. В редакторе контента нажмите название темы (Classic) или меню с тремя точками на активности (New Content Experience).
2. Установите статус **Draft** (Classic) или выключите **Visibility** (New Content Experience).

Draft-темы невидимы студентам. Инструкторы и TA по-прежнему видят их с бейджем "Draft".

Чтобы ограничить тему для определённой группы или секции:

1. Откройте тему.
2. Нажмите меню названия темы > **Edit Properties In-place** (Classic) или **Edit** > **Restrictions** (New Content Experience).
3. В разделе **Release Conditions** нажмите **Create**.
4. Выберите **Group enrollment** или **Section enrollment**, выберите группу/секцию и сохраните.

Условия выпуска работают вместе с собственной системой сопоставления ролей FastComments. Студенты, которые не видят тему, не получают LTI launch.

#### What Students See on First Launch

Когда студент нажимает тему (или загружает HTML-тему с embed):

1. Brightspace выполняет LTI 1.3 launch в фоне.
2. FastComments получает имя студента, email, URL аватара и роль LMS и автоматически авторизует их. Запроса на вход в FastComments не будет.
3. Поток комментариев для этого resource link отображается внутри Brightspace iframe.

Сопоставление ролей при запуске:

- Brightspace `Administrator` становится в FastComments администратором (admin) для thread (полный доступ к модерации, удалению, бану и настройкам).
- Brightspace `Instructor` становится в FastComments модератором (moderator) (pin, hide, delete, ban).
- Все остальные роли (`Learner`, `TeachingAssistant`, и т. д.) становятся стандартными комментаторами.

Комментарии приписываются к учётной записи студента в Brightspace. Если студент изменит своё имя или аватар в Brightspace, при следующем LTI launch изменения синхронизируются.

#### Iframe Height and Resize

FastComments отправляет postMessage `org.imsglobal.lti.frameResize` при каждом рендере thread и при изменениях контента (новый комментарий, разворачивание ответов). Brightspace слушает это сообщение и регулирует высоту iframe, чтобы поток не обрезался и внутри не появлялась полоса прокрутки.

Если iframe остаётся фиксированной маленькой высоты:

- Убедитесь, что курс загружается по HTTPS. Слушатель postMessage Brightspace отклоняет фреймы с mixed-content.
- Убедитесь, что никакое расширение браузера не блокирует канал postMessage.
- Для inline embed в HTML-теме окружающая HTML-разметка не должна оборачивать iframe в контейнер с фиксированной высотой. Уберите любой inline `style="height: ..."` у родительского элемента.

#### Brightspace-Specific Gotchas

**Tool not showing in the Add Existing picker.** Deployment не включён для org unit этого курса. Администратору нужно добавить org unit (или родительский) в список **Org Units** деплоя. Регистрация инструмента сама по себе недостаточна; scope deployment определяет, в каких курсах инструмент видим.

**`deployment_id` mismatch on launch.** FastComments использует TOFU и фиксирует первый `deployment_id`, который он увидел при регистрации. Если администратор удаляет исходный deployment и создаёт новый, запуски из нового deployment отклоняются с ошибкой несовпадения deployment. Решение — перерегистрировать FastComments (сгенерировать новый registration URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">get it here</a>) и снова выполнить Dynamic Registration); старая запись конфигурации будет заменена.

**Tool launches but shows "Invalid LTI launch".** Курс находится в другой структуре tenant/org, чем покрывает deployment, или deployment был отключён после регистрации. Проверьте **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > переключатель **Enabled** и список org unit в deployment.

**Names and roles missing inside FastComments.** Brightspace отправляет LTI-запуски с Names and Role Provisioning Services (NRPS) claims. Если курс был обновлён из старой LTI 1.1 ссылки, в launch отсутствуют `name` и `email` claims. Пере-добавьте тему FastComments через **Add Existing** (не мигрируйте старую ссылку), чтобы запуск использовал LTI 1.3.

**Embed shows a login screen instead of auto-SSO.** HTML-тема была вставлена как обычный `<iframe>`, указывающий на FastComments, а не через **Insert Stuff** > **LTI Advantage**. Обычные iframe пропускают LTI launch и попадают пользователей на публичную страницу FastComments. Удалите iframe и вставьте заново через поток Insert Stuff.