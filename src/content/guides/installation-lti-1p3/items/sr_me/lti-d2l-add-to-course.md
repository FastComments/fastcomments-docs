Oва страница покрива додавање FastComments у Brightspace курс након што је администратор регистровао алат и креирао deployment. Ако алат још није регистрован, прво погледајте D2L упутство за регистрацију.

<div class="screenshot white-bg">
    <div class="title">FastComments уграђен као тема јединице у Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments running inside a Brightspace unit, showing threaded comments and an @-mention picker" />
</div>

Brightspace испоручује два окружења за креирање садржаја: **Classic Content** и **New Content Experience** (такође названо **Lessons**). Обе опције омогућавају FastComments, али се путање у менију разликују. Сваки одељак доле обухвата оба када се разликују.

#### Locate the FastComments Tool

FastComments алат се појављује на два места у уређивачу садржаја курса:

1. У activity picker-у, до ког се долази преко дугмета **Add Existing** у модулу/јединици (у старијим верзијама Brightspace-а означено као **Add Existing Activities**). FastComments се у новијим издањима појављује директно у picker-у; старије верзије га стављају у подмени **External Learning Tools**. Било која путања додаје FastComments као самосталну тему.
2. У дијалогу **Insert Stuff** унутар HTML уређивача, под **LTI Advantage**. Ово уграђује FastComments inline у HTML тему преко LTI deep linking тока.

Ако FastComments није присутан ни у једном picker-у, deployment није омогућен за org unit који садржи курс. Замolite свог Brightspace администратора да отвори **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, отвори deployment и додa курсев org unit (или родитељски org unit) под **Org Units**.

#### Add FastComments as a Topic in a Module

Classic Content:

1. Отворите курс и кликните **Content** у навбару.
2. Изаберите модул који треба да садржи дискусију (или креирајте нови преко **Add a module**).
3. Кликните **Add Existing** (старији Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. У picker-у, кликните **FastComments**. Brightspace креира тему у модулу и враћа вас на приказ садржаја.
5. Кликните нову тему. Преименујте је у нешто описно попут `FastComments Discussion` користећи inline title editor.

New Content Experience (Lessons):

1. Отворите курс и кликните **Content**.
2. Отворите јединицу и lesson који треба да садржи дискусију.
3. Кликните **Add** > **Existing Activity** и изаберите **FastComments** (старији Brightspace: унутар **External Learning Tools**).
4. Активност се додаје у lesson.
5. Кликните на наслов активности да бисте је преименовали.

Кад било који корисник (инструктор или студент) први пут отвори тему, FastComments иницијализује thread за тај resource link. Thread је везан за resource link ID, па промена имена или премештање теме не мења који thread се учитава.

#### Embed FastComments Inline in an HTML Topic

Користите овај ток када желите да коментари буду испод читања, видеа или другог садржаја унутар исте странице теме уместо као посебна тема.

1. Отворите или креирајте HTML тему у модулу/lesson-у.
2. Кликните **Edit HTML** да отворите Brightspace HTML уређивач.
3. Поставите курсор на место где треба да се појави thread коментара.
4. Кликните дугме **Insert Stuff** (икона слагалице у траци алатки уређивача).
5. У дијалогу Insert Stuff, скролујте до **LTI Advantage** и кликните **FastComments**.
6. FastComments отвара deep linking picker. Потврдите поставку (подразумевана подешавања раде за дискусије о садржају); кликните **Insert** или **Continue**.
7. Brightspace се враћа у HTML уређивач са placeholder блоком који представља LTI launch. Кликните **Save and Close** на теми.

Када се тема учита, Brightspace замењује placeholder iframe-ом који аутоматски покреће FastComments преко LTI. Студенти виде thread дискусије inline.

Једна HTML тема може да садржи више deep-linked FastComments уградњи. Свака уградња добија свој thread јер сваки deep link генерише јединствени resource link ID.

#### Module Topic vs Inline Quicklink

Изаберите приступ са **module topic** када:

- Дискусија је основна активност за тај корак у модулу.
- Желите да тема буде приказана у Brightspace-овом садржачном индексу, праћењу завршетка и Class Progress.

Изаберите приступ са **inline embed** када:

- Коментари треба да стоје испод другог садржаја на истој страници.
- Не желите посебну ставку која се прати за завршетак у садржају.

#### Visibility, Draft, and Release Conditions

Нова FastComments тема је по подразумеваној вредности видљива студентима. Да бисте је сакрили док је подешавате:

1. У уређивачу садржаја, кликните на наслов теме (Classic) или три тачке у менију активности (New Content Experience).
2. Подесите статус на **Draft** (Classic) или искључите **Visibility** (New Content Experience).

Draft теме су невидљиве студентима. Инструктори и ТА и даље виде тему са ознаком "Draft".

Да бисте ограничили тему на специфичну групу или секцију:

1. Отворите тему.
2. Кликните мени на наслову теме > **Edit Properties In-place** (Classic) или **Edit** > **Restrictions** (New Content Experience).
3. Под **Release Conditions**, кликните **Create**.
4. Изаберите **Group enrollment** или **Section enrollment**, одаберите групу/секцију и сачувајте.

Release conditions се комбинују са FastComments-овим приступом по улогама. Студенти који не могу видети тему не добијају LTI launch.

#### What Students See on First Launch

Када студент кликне тему (или учита HTML тему са уградњом):

1. Brightspace изводи LTI 1.3 launch у позадини.
2. FastComments прими студентово име, имејл, URL аватара и LMS улогу, и аутоматски их пријави. Нема FastComments прозора за пријаву.
3. Thread коментара за тај resource link се рендерује унутар Brightspace iframe-а.

Mapирање улога при launch-у:

- Brightspace `Administrator` постаје FastComments **admin** за thread (пун приступ модерацији, брисању, бановању и конфигурацији).
- Brightspace `Instructor` постаје FastComments **moderator** (pin, hide, delete, ban).
- Све остале улоге (`Learner`, `TeachingAssistant`, итд.) постају стандардни коментатори.

Коментари се приписују студентовом Brightspace налогу. Ако студент измени своје име или аватар у Brightspace-у, следећи LTI launch синхронизује промену.

#### Iframe Height and Resize

FastComments емитује `org.imsglobal.lti.frameResize` postMessage при сваком рендеру thread-а и при променама садржаја (нов коментар, проширене одговоре). Brightspace слуша за ову поруку и прилагођава висину iframe-а тако да thread не буде исечен и да не прикаже унутрашњи scrollbar.

Ако iframe остане на фиксно малој висини:

- Потврдите да је курс учитан преко HTTPS-а. Brightspace-ов слушалац за postMessage одбацује frames мешовитог садржаја.
- Потврдите да ниједан browser extension не блокира postMessage канал.
- За inline уградње у HTML теми, околни HTML не сме да ставља iframe у контейнер фиксне висине. Уклоните било који inline `style="height: ..."` са родитељског елемента.

#### Brightspace-Specific Gotchas

**Tool not showing in the Add Existing picker.** Deployment није омогућен за org unit овог курса. Администратор треба да додa org unit (или родитеља) у листу **Org Units** за deployment. Самo регистрација алата није довољна; deployment одређује који курсеви виде алат.

**`deployment_id` mismatch on launch.** FastComments TOFU-pins први `deployment_id` који види за регистрацију. Ако администратор обрише оригинални deployment и направи нови, покретања са новог deployment-а ће бити одбијена са грешком о неслагању deployment-а. Решење је поново регистровати FastComments (генеришите нови registration URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">get it here</a>) и покрените Dynamic Registration поново); стари конфигурациони запис се замењује.

**Tool launches but shows "Invalid LTI launch".** Курс је у другој структури tenant/org него што deployment покрива, или је deployment онемогућен након регистрације. Проверите **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** toggle и листу org unit-а у deployment-у.

**Names and roles missing inside FastComments.** Brightspace шаље LTI launch-ове са Names and Role Provisioning Services (NRPS) claims. Ако је курс ажуриран са старије LTI 1.1 везе, launch може да нема `name` и `email` claims. Поново додајте FastComments тему преко **Add Existing** (не мигрирајте стару везу) тако да launch користи LTI 1.3.

**Embed shows a login screen instead of auto-SSO.** HTML тема је уметнута као обичан `<iframe>` који показује директно на FastComments уместо преко **Insert Stuff** > **LTI Advantage**. Обични iframe-ови прескачу LTI launch и воде кориснике на јавну FastComments страницу. Обришите iframe и поново уметните преко Insert Stuff тока.