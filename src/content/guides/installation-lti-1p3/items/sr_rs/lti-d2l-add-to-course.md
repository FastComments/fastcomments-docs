Ova страница обухвата додавање FastComments у Brightspace курс након што администратор региструје алат и креира deployment. Ако алат још није регистрован, прво погледајте D2L registration guide.

<div class="screenshot white-bg">
    <div class="title">FastComments embedded as a unit topic in Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments running inside a Brightspace unit, showing threaded comments and an @-mention picker" />
</div>

Brightspace испоручује два окружења за ауторство садржаја: **Classic Content** и **New Content Experience** (такође познато као **Lessons**). Оба укључују FastComments, али путање у менију се разликују. Сваки следећи одељак покрива оба где се разликују.

#### Locate the FastComments Tool

FastComments алат се појављује на два места у уређивачу курса:

1. У activity picker-у, доступном преко дугмета **Add Existing** модула/јединства (означено као **Add Existing Activities** у старијим верзијама Brightspace). FastComments се у актуелним верзијама директно појављује у picker-у; у старијим верзијама налази се унутар подменија **External Learning Tools**. Било који од ова два пута додаје FastComments као самосталну тему.
2. У дијалогу **Insert Stuff** унутар HTML уређивача, под **LTI Advantage**. Ово уграђује FastComments inline у HTML тему преко LTI deep linking флоуа.

Ако FastComments не видите ни у једном picker-у, deployment није омогућен за org unit који садржи курс. Затражите од Вашег Brightspace администратора да отвори **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, отвори deployment и дода курсев org unit (или родитељски org unit) под **Org Units**.

#### Add FastComments as a Topic in a Module

Classic Content:

1. Отворите курс и кликните **Content** у навбару.
2. Изаберите модул који треба да садржи дискусију (или креирајте један преко **Add a module**).
3. Кликните **Add Existing** (старији Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. У picker-у кликните **FastComments**. Brightspace креира тему у модулу и враћа вас у content view.
5. Кликните нову тему. Преименујте је у нешто описно као `FastComments Discussion` помоћу inline title уређивача.

New Content Experience (Lessons):

1. Отворите курс и кликните **Content**.
2. Отворите unit и lesson који треба да садрже дискусију.
3. Кликните **Add** > **Existing Activity** и изаберите **FastComments** (старији Brightspace: уграђено под **External Learning Tools**).
4. Активност се додаје у lesson.
5. Кликните на наслов активности да бисте га преименовали.

Први пут када било који корисник (предавач или студент) отвори тему, FastComments иницијализује thread за ту resource link. Thread је везан за resource link ID, тако да преименовање или премештање теме не мења који thread се учитава.

#### Embed FastComments Inline in an HTML Topic

Користите овај флоу када желите да коментари буду испод текста за читање, видеа или другог садржаја унутар исте странице теме уместо као посебна тема.

1. Отворите или креирајте HTML тему у модулу/lesson-у.
2. Кликните **Edit HTML** да отворите Brightspace HTML уређивач.
3. Поставите курсор тамо где треба да се појави comment thread.
4. Кликните дугме **Insert Stuff** (икона коцкице у toolbar-у уређивача).
5. У дијалогу Insert Stuff скролујте до **LTI Advantage** и кликните **FastComments**.
6. FastComments отвара deep linking picker. Потврдите постављање (подразумеване опције раде за content discussions); кликните **Insert** или **Continue**.
7. Brightspace се враћа у HTML уређивач са placeholder блоком који представља LTI launch. Кликните **Save and Close** на теми.

Када се тема учита, Brightspace замењује placeholder iframe-ом који аутоматски покреће FastComments преко LTI. Студенти виде дискусију inline.

Једна HTML тема може да садржи више deep-linked FastComments embed-ова. Сваки embed добија свој thread јер сваки deep link производи јединствени resource link ID.

#### Module Topic vs Inline Quicklink

Изаберите приступ са **module topic** када:

- Дискусија је примарна активност за тај корак у модулу.
- Желите да тема буде видљива у Brightspace-овом table of contents, completion tracking и Class Progress.

Изаберите приступ са **inline embed** када:

- Коментари треба да стоје испод другог садржаја на истој страници.
- Не желите посебну ставку која се прати по напредовању у table of contents.

#### Visibility, Draft, and Release Conditions

Нова FastComments тема је подразумевано видљива студентима. Да бисте је сакрили док је подешавате:

1. У content уређивачу кликните на наслов теме (Classic) или на мени са три тачке на активности (New Content Experience).
2. Подесите статус на **Draft** (Classic) или искључите **Visibility** (New Content Experience).

Draft теме су невидљиве студентима. Инструктори и ТА-и их и даље виде са ознаком "Draft".

Да бисте ограничили тему на одређену групу или секцију:

1. Отворите тему.
2. Кликните мени на наслову теме > **Edit Properties In-place** (Classic) или **Edit** > **Restrictions** (New Content Experience).
3. Под **Release Conditions**, кликните **Create**.
4. Изаберите **Group enrollment** или **Section enrollment**, одаберите групу/секцију и сачувајте.

Release conditions се сложе са FastComments-овим сопственим role mapping-ом. Студенти који не могу да виде тему не добијају LTI launch.

#### What Students See on First Launch

Када студент кликне тему (или учита HTML тему са embed-ом):

1. Brightspace изводи LTI 1.3 launch у позадини.
2. FastComments прима студентово име, email, avatar URL и LMS улогу, и аутоматски их пријављује. Нема FastComments захтева за пријављивање.
3. Comment thread за тај resource link се приказује унутар Brightspace iframe-а.

Role mapping при launch-у:

- Brightspace `Administrator` постаје FastComments **admin** за thread (пуна модерација, брисање, бан и приступ конфигурацији).
- Brightspace `Instructor` постаје FastComments **moderator** (pin, hide, delete, ban).
- Све остале улоге (`Learner`, `TeachingAssistant`, итд.) постају стандардни коментатори.

Коментари се приписују студентовом Brightspace налогу. Ако студент промени име или аватар у Brightspace-у, следећи LTI launch синхронизује измене.

#### Iframe Height and Resize

FastComments емитује `org.imsglobal.lti.frameResize` postMessage при сваком render-у thread-а и при променама садржаја (нови коментар, проширене одговоре). Brightspace слуша за ову поруку и прилагођава висину iframe-а тако да thread не буде исечен и да се не појави унутрашњи scrollbar.

Ако iframe остане на фиксно малој висини:

- Потврдите да је курс учитан преко HTTPS. Brightspace-ов postMessage listener одбацује mixed-content фреймове.
- Потврдите да ниједан browser extension не блокира postMessage канал.
- За inline embed-ове у HTML теми, околни HTML не сме да умотава iframe у контејнер фиксне висине. Уклоните било који inline `style="height: ..."` из родитељског елемента.

#### Brightspace-Specific Gotchas

**Tool not showing in the Add Existing picker.** Deployment није омогућен за org unit овог курса. Администратор треба да дода org unit (или родитеља) на листу Org Units у deployment-у. Само регистровање алата није довољно; deployment одређује који курсеви виде алат.

**`deployment_id` mismatch on launch.** FastComments TOFU-пинова први `deployment_id` који види за registration. Ако администратор избрише оригинални deployment и креира нови, launch-еви из новог deployment-а се одбацују са deployment mismatch грешком. Решење је поново регистровати FastComments (генеришите нову registration URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">get it here</a>) и покрените Dynamic Registration опет); стари конфигурациони запис се замењује.

**Tool launches but shows "Invalid LTI launch".** Курс је у другом tenant/org структури него што deployment покрива, или је deployment био онемогућен након регистрације. Поново проверите **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** toggle и листу org unit-ova у deployment-у.

**Names and roles missing inside FastComments.** Brightspace шаље LTI launches са Names and Role Provisioning Services (NRPS) claims. Ако је курс надограђен са старе LTI 1.1 везе, launch може да нема `name` и `email` claims. Поново додайте FastComments тему преко **Add Existing** (не мигрирајте стару везу) тако да launch користи LTI 1.3.

**Embed shows a login screen instead of auto-SSO.** HTML тема је уметнута као обичан `<iframe>` који показује директно на FastComments уместо преко **Insert Stuff** > **LTI Advantage**. Обични iframe-ови прескачу LTI launch и користитеље воде на јавно доступну FastComments страницу. Обришите iframe и поново уметните преко Insert Stuff флоуа.