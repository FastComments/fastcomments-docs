Ова страница обухвата додавање FastComments у Brightspace курс након што је администратор регистровао алат и креирао деплојмент. Ако алат још није регистрован, прво погледајте D2L водич за регистрацију.

Brightspace испоручује два искуства за креирање садржаја: **Classic Content** и **New Content Experience** (такође звано **Lessons**). Обе опције пружају приступ FastComments, али путеви кроз мени се разликују. Свако од следећих одељака покрива оба начина тамо где се разликују.

#### Locate the FastComments Tool

FastComments алат се појављује на два места унутар едитора садржаја курса:

1. Изборник активности, до којег се приступа преко дугмета **Add Existing** модула/јединке (у старијим верзијама Brightspace означено као **Add Existing Activities**). FastComments се у новим издањима директно појављује у изборнику; старије верзије га смештају у подменију **External Learning Tools**. Сваки од наведених путева додаје FastComments као самосталну тему.
2. Дијалог **Insert Stuff** унутар HTML едитора, под **LTI Advantage**. Ово уграђује FastComments инлајн у HTML тему путем LTI deep linking флоуа.

Ако FastComments није присутан ни у једном изборнику, деплојмент није омогућен за организациону јединицу која држи курс. Замолите вашег Brightspace администратора да отвори **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, отвори деплојмент и дода курсеву организациону јединицу (или родитељску орг јединицу) у одељак **Org Units**.

#### Add FastComments as a Topic in a Module

Classic Content:

1. Отворите курс и кликните **Content** у навбару.
2. Изаберите модул који треба да садржи дискусију (или креирајте један преко **Add a module**).
3. Кликните **Add Existing** (старији Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. У изборнику кликните **FastComments**. Brightspace креира тему у модулу и враћа вас у приказ садржаја.
5. Кликните на нову тему. Преименујте је у нешто описно попут `FastComments Discussion` користећи уређивач на месту насловa.

New Content Experience (Lessons):

1. Отворите курс и кликните **Content**.
2. Отворите јединицу и lesson који треба да садржи дискусију.
3. Кликните **Add** > **Existing Activity** и изаберите **FastComments** (старији Brightspace: уграђено под **External Learning Tools**).
4. Активност се дода у lesson.
5. Кликните на наслов активности да бисте је преименовали.

Први пут када било који корисник (инструктор или студент) отвори тему, FastComments иницијализује нит за ту resource link. Нит је везана за resource link ID, тако да променa имена или премештање теме не мења која нит се учитава.

#### Embed FastComments Inline in an HTML Topic

Користите овај флоу када желите да коментари буду испод читања, видеа или другог садржаја унутар исте странице теме уместо као посебна тема.

1. Отворите или креирајте HTML тему у модулу/lesonu.
2. Кликните **Edit HTML** да отворите Brightspace HTML едитор.
3. Поставите курсор на место где треба да се појави нит коментара.
4. Кликните дугме **Insert Stuff** (икона слагалице у алатној траци едитора).
5. У дијалогу Insert Stuff скролујте до **LTI Advantage** и кликните **FastComments**.
6. FastComments отвара deep linking изборник. Потврдите место постављања (подразумеване опције раде за дискусије садржаја); кликните **Insert** или **Continue**.
7. Brightspace се враћа у HTML едитор са плейсхолдер блоком који представља LTI лонч. Кликните **Save and Close** на теми.

Када се тема учита, Brightspace замењује плейсхолдер iframe-ом који аутоматски покреће FastComments преко LTI. Студенти виде нит дискусије инлајн.

Једна HTML тема може да садржи више deep-linked FastComments уградњи. Свака уградња добија своју нит јер сваки deep link генерише јединствени resource link ID.

#### Module Topic vs Inline Quicklink

Изаберите приступ кроз **module topic** када:

- Дискусија је примарна активност за тај корак у модулу.
- Желите да тема буде видљива у Brightspace табелама садржаја, праћењу завршетка и Class Progress.

Изаберите приступ **inline embed** када:

- Коментари треба да стоје испод другог садржаја на истој страници.
- Не желите посебну ставку праћења завршетка у табли садржаја.

#### Visibility, Draft, and Release Conditions

Нова FastComments тема је по подразумеваној поставци видљива студентима. Да бисте је сакрили док је подешавате:

1. У едитору садржаја, кликните на наслов теме (Classic) или три тачке мени на активности (New Content Experience).
2. Поставите статус на **Draft** (Classic) или искључите **Visibility** (New Content Experience).

Draft теме су невидљиве студентима. Инструктори и TA-ови и даље их виде са ознаком "Draft".

Да ограничите тему на одређену групу или секцију:

1. Отворите тему.
2. Кликните мени на наслову теме > **Edit Properties In-place** (Classic) или **Edit** > **Restrictions** (New Content Experience).
3. Под **Release Conditions**, кликните **Create**.
4. Изаберите **Group enrollment** или **Section enrollment**, одаберите групу/секцију и сачувајте.

Release conditions се надовезују на FastComments-ово сопствено мапирање улога. Студенти који не могу видети тему не добијају LTI покретање.

#### What Students See on First Launch

Када студент кликне на тему (или учита HTML тему са уградњом):

1. Brightspace извршава LTI 1.3 лонч у позадини.
2. FastComments прима студентово име, е-пошту, URL аватара и LMS улогу, и аутоматски их пријављује. Нема FastComments прозора за пријаву.
3. Нит коментара за тај resource link се рендерује унутар Brightspace iframe-а.

Мапирање улога при покретању:

- Brightspace `Administrator` постаје FastComments **admin** за нит (пун приступ модерацији, брисању, бановању и конфигурацији).
- Brightspace `Instructor` постаје FastComments **moderator** (pin, hide, delete, ban).
- Све друге улоге (`Learner`, `TeachingAssistant`, итд.) постају обични коментатори.

Коментари се приписују студентовом Brightspace налогу. Ако студент измени своје име или аватар у Brightspace-у, следеће LTI покретање синхронизује промену.

#### Iframe Height and Resize

FastComments емитује `org.imsglobal.lti.frameResize` postMessage при сваком рендеровању нити и при променама садржаја (нов коментар, проширени одговори). Brightspace слуша ову поруку и подешава висину iframe-а тако да нит није скраћена и да се не појављује унутрашњи scrollbar.

Ако iframe остаје на фиксној малој висини:

- Потврдите да се курс учитава преко HTTPS-а. Brightspace-ов постMessage слушач одбија frames са мешовитим садржајем.
- Потврдите да ниједан прегледачки екстензија не блокира postMessage канал.
- За inline уградње у HTML теми, околни HTML не сме да умотава iframe у контејнер фиксне висине. Уклоните било који inline `style="height: ..."` из родитељског елемента.

#### Brightspace-Specific Gotchas

**Tool not showing in the Add Existing picker.** Деплојмент није омогућен за организациону јединицу овог курса. Администратор треба да дода организациону јединицу (или родитеља) у списак **Org Units** деплојмента. Сама регистрација алата није довољна; деплојмент одређује који курсеви виде алат.

**`deployment_id` mismatch on launch.** FastComments TOFU-пинује први `deployment_id` који види за регистрацију. Ако администратор избрише оригинални деплојмент и креира нови, покретања са новог деплојмента се одбијају са грешком неслагања деплојмента. Решение је поново регистровати FastComments (генерисати нови registration URL и поновити Dynamic Registration); стара конфигурациона ставка ће бити замењена.

**Tool launches but shows "Invalid LTI launch".** Курс се налази у другој структури tenant/орг него што покрива деплојмент, или је деплојмент био онемогућен након регистрације. Поново проверите **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** прекидач и списак организационих јединица деплојмента.

**Names and roles missing inside FastComments.** Brightspace шаље LTI покретања са Names and Role Provisioning Services (NRPS) тврдњама. Ако је курс надограђен из старије LTI 1.1 везе, покретање може да нема `name` и `email` тврдње. Поново додајте FastComments тему преко **Add Existing** (не мигрирајте стару везу) тако да покретање користи LTI 1.3.

**Embed shows a login screen instead of auto-SSO.** HTML тема је уметнута као обични `<iframe>` који показује на FastComments уместо преко **Insert Stuff** > **LTI Advantage**. Обични iframe-ови прескачу LTI покретање и доводе кориснике на јавну FastComments страницу. Избришите iframe и поново уметните преко Insert Stuff флоуа.