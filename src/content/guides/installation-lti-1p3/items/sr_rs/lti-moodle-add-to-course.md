Овај водич описује додавање FastComments у Moodle 4.x курс након што је администратор сајта регистровао алат и подешавао да се приказује у activity chooser-у. Ако FastComments још није регистрован, прво погледајте Moodle registration guide.

#### Open the Course in Edit Mode

1. Пријавите се у Moodle као Editing Teacher (или виши) за тај курс.
2. Отворите курс.
3. Укључите **Edit mode** помоћу прекидача у горњем десном углу заглавља курса.

Moodle 4.x је заменио застарели падајући мени "Add an activity or resource" који је користио 3.x пуноекранским дијалогом за избор активности (activity chooser). Moodle 4.5 задржава исти chooser али додаје ред омилених (starred/favorites) на врху, тако да једном када закачите FastComments биће брже доступан у наредним одељцима.

#### Add the FastComments Activity

1. Скролујте до одељка курса (topic или week) у који припада дискусија.
2. Кликните на **Add an activity or resource** на дну тог одељка.
3. У chooser дијалогу изаберите **FastComments**. Ако га не видите, прескочите у одељак са проблемима (gotchas) испод.

Отвара се форма за подешавање активности. Поља која су важна:

- **Activity name** (required). Приказује се на страници курса и у gradebook-у. Пример: `Week 3 Discussion`.
- **Activity description**. Опционо уводно излагање које се рендерује изнад нити коментара.
- **Show description on course page**. Означите ако желите да опис буде видљив без клика на активност.
- **Preconfigured tool**. Постављено на `FastComments` (аутоматски изабрано када је покренуто из chooser-а). Не мењајте.
- **Launch container**. Поставите на **New window**. Погледајте одељак са проблемима (gotchas) зашто „Same window“ може отказати у неким Moodle инсталацијама.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. Оставите празно. Dynamic Registration је ово руковао на нивоу сајта.

Скролујте доле и кликните **Save and return to course** (или **Save and display** да одмах отворите активност).

Активност се појављује као ред у одељку са FastComments иконцом. Студенти кликају на ред да отворе нит коментара.

#### Embed FastComments Inline with the Editor

За нит унутар Page, Book chapter, Lesson или било ког другог ресурса који користи Atto или TinyMCE едитор:

1. Отворите ресурс у режиму уређивања.
2. Поставите курсор на место где нит треба да се појави.
3. У траци алата едитора кликните на **LTI** / **External tool** дугме. У Atto-у је означено као "Insert LTI Advantage content". У TinyMCE-у (подразумевано у Moodle 4.3+) налази се у менију **More** као **External tools**.
4. Изаберите **FastComments** са листе алата.
5. FastComments отвори deep-linking picker. Потврдите наслов нити и кликните **Embed**.
6. Едитор убацује LTI placeholder блок. Сачувајте ресурс.

Сваки уграђени инстанц је одвојена нит кључована на deep-link content item ID, тако да Page са три FastComments уграђена добија три независне нити.

#### Restrict Access and Group Settings

Стандардна Moodle подешавања активности важе и за FastComments активности:

- **Common module settings** > **Group mode**. Постављање на **Separate groups** или **Visible groups** само по себи не раздваја FastComments у по-групне нити. Moodle-ов group mode само филтрира gradebook и листу чланова. Да бисте водили појединачну нит по групи, додајте по једну FastComments активност по групи и користите **Restrict access** да ограничите опсег сваке.
- **Restrict access** > **Add restriction**. Подржава стандардне Moodle услове: **Date**, **Grade**, **Group**, **Grouping**, **User profile**, и угнежђене скупове ограничења. Користите **Group** да закључате FastComments активност за једну групу.
- **Activity completion**. Поставите на **Students must view this activity to complete it** ако желите праћење завршетка. FastComments тренутно не извештава о догађају завршетка назад у Moodle осим лансирања.

#### Role Mapping

FastComments чита LTI `roles` claim који Moodle шаље при сваком лансирању и мапира га овако:

- Moodle **Manager** or **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** or **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> read-only

Администратори могу да бришу било који коментар, забране кориснике и уређују подешавања нити. Модератори могу да бришу и одобравају коментаре унутар нити у коју су лансирани. Прилагођене Moodle улоге наслеђују мапирање архетипа од којег су клониране.

#### What Students See

Студенти кликају FastComments активност (или скролују до уграђеног блока унутар Page или Book). Moodle шаље њихов идентитет FastComments-у преко LTI лансирања:

- Нема екрана за пријаву. FastComments их пријављује користећи Moodle налог.
- Њихово приказно име, имејл и аватар долазе из Moodle-а.
- Нит је скопована на (Moodle site, course, resource link ID), тако да иста активност дуплирана у другом курсу добија свежу нит.
- Укључене су thread-ови одговора, гласање и нотификације као у самосталној FastComments нити.

#### Lock Down Public Access (Recommended)

По подразумевану, FastComments подаци коментара су јавни за читање. Сваки ко може да погодии URL нити или API endpoint може да види коментаре, чак и ван Moodle-а. За дискусије у курсу готово сигурно желите да ограничите преглед само на уписане студенте.

Отворите вашу <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget customization page</a> и креирајте правило са **Require SSO To View Comments** омогућеним, затим поставите security level на **Secure SSO** тако да нити могу да се учитавају само преко потписаног LTI лансирања.

Погледајте [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) за комплетан корак-по-корак водич, укључујући како да ограничите правило на један домен или страницу.

#### Moodle Gotchas

**FastComments missing from the activity chooser.** Администратор сајта је регистровао алат али није подесио **Tool configuration usage** на **Show in activity chooser and as a preconfigured tool**. Поправите то у **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > икона зупчаника на FastComments плочици.

**Launch fails or shows a blank frame when set to "Same window".** Moodle-ове session cookies користе `SameSite=Lax` по подразумевaњу, и неки прегледачи их уклањају при cross-site POST захтеву који LTI 1.3 користи да се врати из FastComments. Поставите **Launch container** на **New window** за активност. Ово је тврдо правило за уграђени FastComments унутар Page или Book, пошто пут лансирања уграђен у едитор увек отвара нови прозор.

**The `iss` claim is the Moodle site URL, not a tenant ID.** FastComments користи Moodle site URL (вредност `wwwroot` у конфигурацији) као LTI issuer. Ако ваш Moodle премести на нови домен или промените `wwwroot`, постојеће FastComments нити остају везане за старог issuer-а и неће се поклапати са новим лансирањима. Поново региструјте алат у односу на нови URL и мигрирајте нити преко FastComments администрације ако је потребно.

**Activity backup and restore.** Бекаповање курса и његово враћање у нови курс креира нове resource link ID-еве, тако да враћене FastComments активности почињу са празним нитима. Оригинални курс задржава оригиналне нити. Ово је очекивано понашање, а не грешка.

**Moodle 4.5 TinyMCE default.** Moodle 4.5 долази са TinyMCE као подразумеваним едитором за нове инсталације. Локација дугмета External tool је у менију **More** (`...`) уместо главне траке алата. Стари сајтови који су надограђени са 4.1 задржавају Atto осим ако администратор није променио подразумевано.

---