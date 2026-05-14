Ова страница покрива додавање FastComments у Brightspace курс након што администратор региструје алат и креира deployment. Ако алат још није регистрован, најприје погледајте D2L упутство за регистрацију.

Brightspace има два искуства за креирање садржаја: **Classic Content** и **New Content Experience** (такође звано **Lessons**). Обе опције пружају FastComments, али путање у менију се разликују. Сваки одељак испод покрива оба где се разилазе.

#### Locate the FastComments Tool

Alat FastComments се појављује на двије мјеста у уређивачу садржаја курса:

1. Picker за активности, доступан преко дугмета **Add Existing** модула/јединце (у старијим верзијама Brightspace означено као **Add Existing Activities**). FastComments се појављује директно у picker-у у тренутним издањима Brightspace; старије верзије га урачунавају под подменијем **External Learning Tools**. Било којом путањом додаје се FastComments као самостална тема.
2. Дијалог **Insert Stuff** унутар HTML уређивача, под **LTI Advantage**. Ово уграђује FastComments inline у HTML тему преко LTI deep linking протока.

Ако FastComments неће да се појави ни у једном picker-у, deployment није омогућен за органску јединицу која садржи курс. Замолите Brightspace администратора да отвори **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, отвори deployment и дода курсну органску јединицу (или родитељску органску јединицу) под **Org Units**.

#### Add FastComments as a Topic in a Module

Classic Content:

1. Отворите курс и кликните **Content** у навбару.
2. Одаберите модул који треба да садржи дискусију (или креирајте један преко **Add a module**).
3. Кликните **Add Existing** (старији Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. У picker-у, кликните **FastComments**. Brightspace креира тему у модулу и враћа вас у приказ садржаја.
5. Кликните нову тему. Преименујте је у нешто описно као `FastComments Discussion` користећи inline уређивач наслова.

New Content Experience (Lessons):

1. Отворите курс и кликните **Content**.
2. Отворите јединцу и lesson који треба да садржи дискусију.
3. Кликните **Add** > **Existing Activity** и одаберите **FastComments** (старији Brightspace: уграђено под **External Learning Tools**).
4. Активност се додаје у lesson.
5. Кликните на наслов активности да бисте га преименовали.

Први пут када било који корисник (инструктор или студент) отвори тему, FastComments иницијализује thread за тај resource link. Thread је везан за resource link ID, па преименовање или премјештање теме не мијења који thread се учитава.

#### Embed FastComments Inline in an HTML Topic

Користите овај ток када желите да коментари буду испод читања, видеа или другог садржаја унутар исте теме, а не као посебна тема.

1. Отворите или креирајте HTML тему у модулу/lesson-у.
2. Кликните **Edit HTML** да отворите Brightspace HTML уређивач.
3. Поставите курсор тамо гдје треба да се појави thread коментара.
4. Кликните дугме **Insert Stuff** (икона слагалице у траци са алаткама уређивача).
5. У дијалогу Insert Stuff, скролујте до **LTI Advantage** и кликните **FastComments**.
6. FastComments отвара deep linking picker. Потврдите мјестање (подразумеване опције раде за дискусије о садржају); кликните **Insert** или **Continue**.
7. Brightspace се враћа у HTML уређивач са placeholder блоком који представља LTI launch. Кликните **Save and Close** на теми.

Када се тема учита, Brightspace замјењује placeholder iframe-ом који аутоматски покреће FastComments преко LTI. Студенти виде thread дискусије inline.

Једна HTML тема може да садржи више deep-linked FastComments уроњења. Сваки embed добија свој thread јер сваки deep link производи појединачан resource link ID.

#### Module Topic vs Inline Quicklink

Одаберите приступ **module topic** када:

- Дискусија је примарна активност за тај корак у модулу.
- Желите да тема буде видљива у Brightspace таблици садржаја, праћењу завршетка и Class Progress.

Одаберите приступ **inline embed** када:

- Коментари треба да буду испод другог садржаја на истој страници.
- Не желите посебну ставку у табли садржаја која се прати по завршетку.

#### Visibility, Draft, and Release Conditions

Нова FastComments тема је по подразумјевању видљива студентима. Да бисте је сакрили док је подешавате:

1. У уређивачу садржаја, кликните на наслов теме (Classic) или три тачке мени на активности (New Content Experience).
2. Подесите статус на **Draft** (Classic) или искључите **Visibility** (New Content Experience).

Draft теме су невидљиве студентима. Инструктори и TA-и их и даље виде са ознаком "Draft".

Да бисте оградили тему на одређену групу или секцију:

1. Отворите тему.
2. Кликните мени на наслову теме > **Edit Properties In-place** (Classic) или **Edit** > **Restrictions** (New Content Experience).
3. Под **Release Conditions**, кликните **Create**.
4. Одаберите **Group enrollment** или **Section enrollment**, изаберите групу/секцију и сачувајте.

Release conditions се употпуњују са FastComments-овим властитим мапирањем улога. Студенти који не могу да виде тему не добијају LTI launch.

#### What Students See on First Launch

Када студент кликне на тему (или учита HTML тему са embed-ом):

1. Brightspace изводи LTI 1.3 launch у позадини.
2. FastComments прими студентово име, е-пошту, URL аватара и LMS улогу, и аутоматски их пријављује. Нема захтјева за пријаву у FastComments.
3. Thread коментара за тај resource link се приказује унутар Brightspace iframe-а.

Mapирање улога при launch-у:

- Brightspace `Administrator` постаје FastComments **admin** за thread (пуна модерација, брисање, ban и приступ конфигурацији).
- Brightspace `Instructor` постаје FastComments **moderator** (pin, hide, delete, ban).
- Све остале улоге (`Learner`, `TeachingAssistant`, итд.) постају стандардни коментатори.

Коментари се приписују студентовом Brightspace налогу. Ако студент измјени своје име или аватар у Brightspace-у, сљедећи LTI launch синхронизује промјену.

#### Iframe Height and Resize

FastComments емитује `org.imsglobal.lti.frameResize` postMessage при сваком рендеровању thread-а и при промјени садржаја (нови коментар, проширени одговори). Brightspace слуша за ову поруку и прилагођава висину iframe-а тако да thread не буде обрезан и да не приказује унутрашњи scrollbar.

Ако iframe остаје фиксне кратке висине:

- Потврдите да се курс учитава преко HTTPS-а. Brightspace-ов listener за postMessage одбацује frames са мешовитим садржајем.
- Потврдите да ниједан прегледачки додатак не блокира postMessage канал.
- За inline embed-ове у HTML теми, околни HTML не смије да умота iframe у контејнер фиксне висине. Уклоните било који inline `style="height: ..."` из родитељског елемента.

#### Brightspace-Specific Gotchas

**Tool not showing in the Add Existing picker.** Deployment није омогућен за органску јединицу овог курса. Администратор треба да дода органску јединицу (или родитеља) у листу **Org Units** деплоијмента. Само регистрација алата није довољна; deployment одређује који курсеви виде алат.

**`deployment_id` mismatch on launch.** FastComments TOFU-пинова први `deployment_id` који види за регистрацију. Ако администратор избрише оригинални deployment и креира нови, покретања са новог deployment-а ће бити одбијена са грешком неусаглашености deployment-а. Рјешење је поново регистровати FastComments (генерисати нову registration URL и поново извршити Dynamic Registration); стара конфигурациона ставка се замењује.

**Tool launches but shows "Invalid LTI launch".** Курс се налази у другачијој структури tenant/org него што покрива deployment, или је deployment онемогућен након регистрације. Поново провјерите **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** тумблер и листу органских јединица deployment-а.

**Names and roles missing inside FastComments.** Brightspace шаље LTI launch-ове са Names and Role Provisioning Services (NRPS) claims. Ако је курс надограђен са старог LTI 1.1 линка, launch може да нема `name` и `email` claims. Поново додајте FastComments тему преко **Add Existing** (не мигрирајте стари линк) тако да launch користи LTI 1.3.

**Embed shows a login screen instead of auto-SSO.** HTML тема је уметнута као обичан <iframe> усмјерен на FastComments уместо преко **Insert Stuff** > **LTI Advantage**. Обични iframe-ови прескачу LTI launch и воде кориснике на јавну FastComments страницу. Обришите iframe и поново уметните преко Insert Stuff тока.