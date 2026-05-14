Ова страница обрађује додавање FastComments у Brightspace курс након што администратор региструје алат и креира deployment. Ако алат још није регистрован, прво погледајте D2L registration guide.

Brightspace испоручује два искуства за креирање садржаја: **Classic Content** и **New Content Experience** (такође названо **Lessons**). Оба приказују FastComments, али путање у менију се разликују. Свако следеће одељак покрива обе где се разликују.

#### Locate the FastComments Tool

FastComments алат се појављује на два места унутар уређивача садржаја курса:

1. Activity picker, доступан преко дугмета **Add Existing** на модули/једици (у старијим верзијама Brightspace-а означено као **Add Existing Activities**). FastComments се појављује директно у picker-у у текућим Brightspace билдовима; старије верзије га уносе под подмени **External Learning Tools**. Сваки од ова два пута додаје FastComments као самосталну тему.
2. Дијалог **Insert Stuff** унутар HTML уређивача, под **LTI Advantage**. Ово уграђује FastComments inline у HTML тему преко LTI deep linking тока.

Ако FastComments не видите ни у једном од picker-ова, deployment није омогућен за org unit који држи курс. Замолите ваш Brightspace администратор да отвори **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, отвори deployment и дода курсев org unit (или родитељски org unit) под **Org Units**.

#### Add FastComments as a Topic in a Module

Classic Content:

1. Отворите курс и кликните **Content** у навбару.
2. Изаберите модул који треба да држи дискусију (или га креирајте преко **Add a module**).
3. Кликните **Add Existing** (старији Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. У picker-у, кликните **FastComments**. Brightspace креира тему у модулу и враћа вас у приказ садржаја.
5. Кликните нову тему. Преименујте је у нешто описно као `FastComments Discussion` користећи inline title editor.

New Content Experience (Lessons):

1. Отворите курс и кликните **Content**.
2. Отворите unit и lesson који треба да држе дискусију.
3. Кликните **Add** > **Existing Activity** и изаберите **FastComments** (старији Brightspace: уграђено под **External Learning Tools**).
4. Активност је додата у lesson.
5. Кликните на наслов активности да бисте га преименовали.

Први пут када било који корисник (инструктор или студент) отвори тему, FastComments иницијализује thread за ту resource link. Thread је везан за resource link ID, тако да преименовање или премештање теме не мења који thread се учитава.

#### Embed FastComments Inline in an HTML Topic

Користите овај ток када желите да коментари буду испод читања, видеа или другог садржаја унутар исте странице теме уместо као посебна тема.

1. Отворите или креирајте HTML тему у модулу/lesson-у.
2. Кликните **Edit HTML** да отворите Brightspace HTML уређивач.
3. Поставите курсор где треба да се појави comment thread.
4. Кликните дугме **Insert Stuff** (икона слагалице у траци уређивача).
5. У дијалогу Insert Stuff, скролујте до **LTI Advantage** и кликните **FastComments**.
6. FastComments отвара deep linking picker. Потврдите позиционирање (подразумеване опције раде за content discussions); кликните **Insert** или **Continue**.
7. Brightspace се враћа у HTML уређивач са placeholder блоком који представља LTI launch. Кликните **Save and Close** на теми.

Када се тема учита, Brightspace замењује placeholder iframe-ом који аутоматски покреће FastComments преко LTI. Студенти виде дискусиони thread inline.

Једна HTML тема може да садржи више deep-linked FastComments embed-ова. Сваки embed добија свој thread јер сваки deep link производи посебан resource link ID.

#### Module Topic vs Inline Quicklink

Изаберите приступ као **module topic** када:

- Дискусија је примарна активност за тај корак у модулу.
- Желите да тема буде видљива у Brightspace-овом table of contents, completion tracking, и Class Progress.

Изаберите приступ као **inline embed** када:

- Коментари треба да стоје испод другог садржаја на истој страници.
- Не желите посебну ставку подлежну праћењу завршетка у table of contents.

#### Visibility, Draft, and Release Conditions

Нова FastComments тема је подразумевано видљива студентима. Да је сакријете док је подешавате:

1. У уређивачу садржаја, кликните на наслов теме (Classic) или на три тачке у менију активности (New Content Experience).
2. Поставите статус на **Draft** (Classic) или искључите **Visibility** (New Content Experience).

Draft теме су невидљиве студентима. Инструктори и ТА-и и даље их виде са "Draft" ознаком.

Да бисте ограничили тему на одређену групу или секцију:

1. Отворите тему.
2. Кликните мени на наслову теме > **Edit Properties In-place** (Classic) или **Edit** > **Restrictions** (New Content Experience).
3. Под **Release Conditions**, кликните **Create**.
4. Изаберите **Group enrollment** или **Section enrollment**, изаберите групу/секцију и сачувајте.

Release conditions се надовезују на FastComments-ову сопствену мапу улога. Студенти који не могу да виде тему не добијају LTI launch.

#### What Students See on First Launch

Када студент кликне тему (или учита HTML тему са embed-ом):

1. Brightspace извршава LTI 1.3 launch у позадини.
2. FastComments прима студентово име, email, avatar URL и LMS улогу, и аутоматски их пријављује. Нема FastComments прозора за пријаву.
3. Comment thread за тај resource link се рендерује унутар Brightspace iframe-а.

Mapирање улога при launch-у:

- Brightspace `Administrator` постаје FastComments **admin** за thread (пуна модерација, брисање, бановање и приступ конфигурацији).
- Brightspace `Instructor` постаје FastComments **moderator** (pin, hide, delete, ban).
- Све остале улоге (`Learner`, `TeachingAssistant`, итд.) постају стандардни коментатори.

Коментари се приписују студентовом Brightspace налогу. Ако студент измени име или avatar у Brightspace-у, следећи LTI launch синхронизује промену.

#### Iframe Height and Resize

FastComments емитује `org.imsglobal.lti.frameResize` postMessage на сваком рендера thread-а и на промене садржаја (нов коментар, проширени одговори). Brightspace слуша ову поруку и прилагођава висину iframe-а тако да thread не буде исечен и да не приказује унутрашњи scrollbar.

Ако iframe остане на фиксно малој висини:

- Потврдите да је курс учитан преко HTTPS. Brightspace-ов слушалац за postMessage одбацује mixed-content frame-ове.
- Потврдите да ниједан browser extension не блокира postMessage канал.
- За inline embed-ове у HTML теми, околни HTML не сме да упакује iframe у контејнер фиксне висине. Уклоните сваки inline `style="height: ..."` са родитељског елемента.

#### Brightspace-Specific Gotchas

**Tool not showing in the Add Existing picker.** Deployment није омогућен за org unit овог курса. Администратор мора да дода org unit (или родитеља) у листу Org Units за deployment. Само регистрација алата није довољна; deployment одређује који курсеви виде алат.

**`deployment_id` mismatch on launch.** FastComments TOFU-пинова први `deployment_id` који види за регистрацију. Ако администратор избрише оригинални deployment и креира нови, покретања из новог deployment-а се одбијају са грешком о неслагању deployment-а. Решење је поновно регистровање FastComments-а (генеришите нови registration URL и покрените Dynamic Registration поново); стари конфигурациони запис се замењује.

**Tool launches but shows "Invalid LTI launch".** Курс се налази у другачијој tenant/org структури него што deployment покрива, или је deployment онемогућен након регистрације. Поново проверите **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** toggle и листу org unit-a deployment-а.

**Names and roles missing inside FastComments.** Brightspace шаље LTI launches са Names and Role Provisioning Services (NRPS) claims. Ако је курс надоградјен са старије LTI 1.1 везе, launch можда нема `name` и `email` claims. Поново додајте FastComments тему преко **Add Existing** (не мигрирајте стару везу) тако да launch користи LTI 1.3.

**Embed shows a login screen instead of auto-SSO.** HTML тема је уметнута као обичан `<iframe>` који показује на FastComments уместо преко **Insert Stuff** > **LTI Advantage**. Обични iframe-ови прескачу LTI launch и кориснике доводе на јавну FastComments страницу. Избришите iframe и поново уметните преко Insert Stuff тока.