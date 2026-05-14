Once FastComments is registered with the platform, instructors add it to course content using the platform's standard external tool flows. This page covers Sakai 23.x and Schoology Enterprise.

#### Sakai

**1. Add FastComments to a site**

Одржаваоц сајта омогућава алат по сајту:

1. Отворите сајт и у левој навигацији кликните **Site Info**.
2. Кликните **Manage Tools**.
3. Скелерајте до листе **External Tools** и укључите **FastComments**.
4. Кликните **Continue**, прегледајте листу алата, затим кликните **Finish**.

FastComments се сада појављује као ставка у левом навигационом менију сајта.

**2. Reorder the left-nav entry**

Идите на **Site Info** > **Tool Order**. Превуците **FastComments** на жељену позицију и кликните **Save**. Са ове странице такође можете преименовати ознаку у навигацији и сакрити је од студената.

**3. Embed inline in a Lessons page**

Да бисте поставили FastComments директно у Lessons страницу уместо као самостални алат у левој навигацији:

1. Отворите алат **Lessons** на сајту.
2. Кликните **Add Content** > **Add External Tool**.
3. Изаберите **FastComments** са листе.
4. Ако је FastComments при регистрацији огласио Deep Linking, Sakai отвори селектор садржаја алата тако да можете одабрати или означити нит. Ако Deep Linking није оглашен, Sakai убацује подразумевани launch линк.
5. Сачувајте ставку у Lessons.

Свака уграђена инстанца добија своју нит, ограничену на тај ресурсни линк.

**4. Permission tweaks for student access**

Sakai штити покретања екстерних алата преко Realms. Да потврдите да студенти могу покренути FastComments:

1. Пријавите се као Sakai администратор и отворите **Administration Workspace** > **Realms**.
2. Отворите релевантни realm (на пример, `!site.template.course` или специфични realm сајта).
3. Потврдите да улога `access` има омогућено `lti.launch` и да су дозволе улоге у групи **external.tools** додељене.
4. Сачувајте realm.

За прескупљења на нивоу сајта, одржаваоц може прилагодити видљивост алата по улогама из **Site Info** > **Tool Order** сакривањем или приказивањем FastComments по улози.

**5. What students see**

Студенти кликну ставку FastComments у левом навигационом менију (или скролују до уграђеног блока у Lessons) и долазе директно у приказ нитованих коментара. SSO је аутоматски: Sakai шаље идентитет корисника у LTI launch и FastComments их пријављује под њиховим Sakai налогом.

Мапирање улога:

- Sakai `Instructor` -> FastComments модератор
- Sakai `Admin` (admin у Administration Workspace) -> FastComments администратор
- Sakai `Student` / `access` -> FastComments коментатор

**6. Sakai gotchas**

- **Tool not visible in Manage Tools.** Ако се FastComments не појављује у листи External Tools, Sakai администратор треба да отвори регистар алата (**Administration Workspace** > **External Tools** > **FastComments**) и подеси **Stealthed** на `false`. Stealthed алати су скривени из пер-сајт Manage Tools избора.
- **Launches breaking in shared-session browsers.** Sakai-јев portal CSRF token је везан за сесију прегледача. Ако је студент пријављен на два Sakai сајта у различитим картицама или има застарелу сесију, launch враћа 403. Решење: затворите друге Sakai картице, одјавите се, поново се пријавите и покрените launch. Администратори могу и повећати `sakai.csrf.token.cache.ttl` ако се ово дешава кластер-широко.
- **Frame embedding.** Потврдите да је `lti.frameheight` у `sakai.properties` довољно велик (600 или више) како нит коментара не би била одсечена унутар Lessons странице.

#### Schoology

Schoology Enterprise има два сценарија инсталације. Потврдите који се односи пре додавања алата у курс.

**1. Two installation scenarios**

- **(a) Enterprise-level install.** Schoology System Administrator је инсталирао FastComments на нивоу организације и доделио га свим курсевима или одређеним шаблонима курсева. Предавачи прескачу инсталацију и иду директно на „Add Materials“.
- **(b) Instructor self-install.** Предавач инсталира алат у један курс преко **Course Options** > **External Tools** > **Install LTI Apps**. Самоинсталација захтева да System Administrator претходно одобри FastComments апликацију на нивоу организације.

**2. Add FastComments as a course material**

Унутар курса:

1. Отворите курс и идите на **Materials**.
2. Кликните **Add Materials** > **Add File/Link/External Tool**.
3. Одаберите **External Tool**.
4. Изаберите **FastComments** са листе регистрованих алата.
5. Поставите **Name** (ово је оно што студенти виде у листи материјала) и опционални **Description**.
6. Оставите **Enable Grading** (grade passback) **OFF**. FastComments не шаље оцене назад у Schoology, па укључивање grade passback ствара празну колону у дневнику оцена.
7. Кликните **Submit**.

Материјал се сада појављује у листи материјала курса и отвара нит FastComments када се кликне.

**3. Inline embedding via the Rich Text editor**

Ако је System Administrator омогућио Deep Linking placement за FastComments током регистрације, предавачи могу уградити нит коментара у било које Rich Text поље (упутства за задатак, тела страница, подстицаји за дискусију):

1. Отворите Rich Text едитор на циљној страници.
2. Кликните икону **External Tool** (пликета) у алатној траци.
3. Изаберите **FastComments**.
4. Конфигуришите уградњу у deep-linking дијалогу и кликните **Insert**.
5. Сачувајте страницу.

Ако дугме External Tool не појављује у Rich Text едитору, Deep Linking је онемогућен за овај алат на овом тенанту. Погледајте напомене у наставку.

**4. Visibility and section assignments**

Schoology ограничава доступност алата по секцијама у оквиру Course Options:

1. Из курса, кликните **Course Options** > **External Tools**.
2. За сваку инсталирану LTI апликацију, контролишете да ли је доступна свим секцијама у курсу или само одређеним секцијама.
3. Да бисте ограничили FastComments само на одређене секције, поништите ознаке оних секција које не би требало да виде алат.
4. Приступ по секцијама такође контролише које секције виде ставку **Add Materials** > **External Tool** за FastComments.

**5. What students see**

Студенти кликну материјал FastComments (или скролују до уграђеног садржаја) и долазе у нитовану дискусију. SSO је аутоматски путем Schoology LTI launch под њиховим Schoology налогом.

Мапирање улога:

- Schoology `Administrator` -> FastComments администратор
- Schoology `Instructor` -> FastComments модератор
- Schoology `Student` -> FastComments коментатор

**6. Schoology gotchas**

- **Enterprise-only.** Лични и бесплатни Schoology налози не могу инсталирати LTI 1.3 алате. Ако је ваш тенант на бесплатном нивоу, опција **External Tools** недостаје у Course Options. Надоградите на Schoology Enterprise да бисте користили FastComments.
- **Deep Linking disabled by tenant default.** Неки Schoology тенанти ограничавају Deep Linking placement на нивоу организације. Када је то случај, предавачи виде само ток **Add Materials** > **External Tool**, а не дугме External Tool у Rich Text едитору. Да би омогућили inline уградњу, System Administrator иде на **System Settings** > **Integration** > **LTI 1.3** > **FastComments** и омогућава **Content Item / Deep Linking** placement, затим сачува.
- **Per-section assignment override.** Ако је FastComments додељен на нивоу организације, али предавач не може да га види у **Add Materials**, секција курса је искључена у организационом додељивању. Замолите System Administrator-а да дода секцију у FastComments апликацију.
- **Material name vs. thread identity.** Преименовање материјала у Schoology-у не помера нит коментара. Нитови су кључирани на LTI resource link ID, па преименовање задржава исту нит; брисање и поновно креирање материјала ствара нову, празну нит.