Ova страница обухвата додавање FastComments у Brightspace курс након што је администратор регистровао алат и креирао deployment. Ако алат још није регистрован, најпре погледајте D2L водич за регистрацију.

<div class="screenshot white-bg">
    <div class="title">FastComments embedded as a unit topic in Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments running inside a Brightspace unit, showing threaded comments and an @-mention picker" />
</div>

Brightspace нуди два начина за креирање садржаја: **Classic Content** и **New Content Experience** (такође названо **Lessons**). Оба приказују FastComments, али се путање у менију разликују. Свако поглавље испод покрива оба случаја тамо где се разликују.

#### Lociranje FastComments алата

FastComments алат се појављује на два места у уреднику садржаја курса:

1. Picker активности, доступан преко дугмета **Add Existing** на модулу/јединици (у старијим верзијама Brightspace означено као **Add Existing Activities**). FastComments се у тренутним верзијама директно појављује у picker-у; у старијим верзијама је унутар подменија **External Learning Tools**. Било који од ових путева додаје FastComments као самосталну тему.
2. Дијалог **Insert Stuff** унутар HTML уредника, под **LTI Advantage**. Ово уграђује FastComments inline у HTML тему путем LTI deep linking флоуа.

Ако FastComments није видљив у ни једном picker-у, deployment није омогућен за организациону јединицу која држи курс. Замолите вашег Brightspace администратора да отвори **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, отвори deployment и дода курсеву организациону јединицу (или родитељску јединицу) под **Org Units**.

#### Додавање FastComments као тему у модул

Classic Content:

1. Отворите курс и кликните **Content** у навбару.
2. Изаберите модул који треба да садржи дискусију (или га креирајте преко **Add a module**).
3. Кликните **Add Existing** (старији Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. У picker-у кликните **FastComments**. Brightspace креира тему у модулу и враћа вас у приказ садржаја.
5. Кликните нову тему. Поново је именујте у нешто описно као `FastComments Discussion` користећи inline title уредник.

New Content Experience (Lessons):

1. Отворите курс и кликните **Content**.
2. Отворите јединицу и lesson који треба да садржи дискусију.
3. Кликните **Add** > **Existing Activity** и изаберите **FastComments** (старији Brightspace: унутар **External Learning Tools**).
4. Активност се додаје у lesson.
5. Кликните на наслов активности да бисте га преименовали.

Први пут када било који корисник (инструктор или студент) отвори тему, FastComments иницијализује нит за ту resource link. Нит је везана за resource link ID, тако да променa имена или премештање теме не мења која нит се учитава.

#### Уграђивање FastComments inline у HTML тему

Користите овај флоу када желите да коментари буду испод читања, видеа или другог садржаја унутар исте странице теме уместо као посебна тема.

1. Отворите или креирајте HTML тему у модулу/lesson-у.
2. Кликните **Edit HTML** да отворите Brightspace HTML уредник.
3. Поставите курсор тамо где треба да се појави нит коментара.
4. Кликните дугме **Insert Stuff** (икона у облику парчета слагалице у траци алата уредника).
5. У дијалогу Insert Stuff скролујте до **LTI Advantage** и кликните **FastComments**.
6. FastComments отвара deep linking picker. Потврдите постављање (подразумеване опције раде за дискусије о садржају); кликните **Insert** или **Continue**.
7. Brightspace се враћа у HTML уредник са placeholder блоком који представља LTI launch. Кликните **Save and Close** на теми.

Када се тема учита, Brightspace замењује placeholder iframe-ом који аутоматски покреће FastComments преко LTI. Студенти виде нит дискусије inline.

Једна HTML тема може да садржи више deep-linked FastComments уградњи. Свака уградња добија своју нит јер сваки deep link генерише јединствени resource link ID.

#### Тема у модулу у односу на inline quicklink

Изаберите приступ са **тема у модулу** када:

- Дискусија је примарна активност за тај корак у модулу.
- Желите да тема буде видљива у табли садржаја Brightspace-а, праћењу завршетка и Class Progress.

Изаберите приступ са **inline уградњом** када:

- Коментари треба да се налазе испод другог садржаја на истој страници.
- Не желите посебну ставку у табли садржаја коју је могуће пратити за завршетак.

#### Видљивост, Draft и услови објављивања

Нова FastComments тема је по подразумеваном видљива студентима. Да бисте је сакрили док је подешавате:

1. У уреднику садржаја кликните на наслов теме (Classic) или на мени са три тачке на активности (New Content Experience).
2. Поставите статус на **Draft** (Classic) или искључите **Visibility** (New Content Experience).

Draft теме су невидљиве студентима. Инструктори и асистенти их и даље виде са ознаком "Draft".

Да бисте ограничили тему на одређену групу или секцију:

1. Отворите тему.
2. Кликните мени на наслову теме > **Edit Properties In-place** (Classic) или **Edit** > **Restrictions** (New Content Experience).
3. Под **Release Conditions**, кликните **Create**.
4. Изаберите **Group enrollment** или **Section enrollment**, одаберите групу/секцију и сачувајте.

Услови објављивања се сабирају са FastComments-овим мапирањем улога. Студенти који не могу да виде тему не добијају LTI launch.

#### Шта студенти виде при првом покретању

Када студент кликне тему (или учита HTML тему са уградњом):

1. Brightspace изводи LTI 1.3 launch у позадини.
2. FastComments прими студентово име, email, avatar URL и LMS улогу, и аутоматски га пријави. Нема FastComments prompt-а за пријаву.
3. Нит коментара за тај resource link се приказује унутар Brightspace iframe-а.

Мапирање улога при покретању:

- Brightspace `Administrator` постаје FastComments **admin** за нит (пуни модераторски приступ, брисање, бан и конфигурација).
- Brightspace `Instructor` постаје FastComments **moderator** (pin, hide, delete, ban).
- Све остале улоге (`Learner`, `TeachingAssistant`, итд.) постају стандардни коментарори.

Коментари се приписују студентовом Brightspace налогу. Ако студент измени своје име или аватар у Brightspace-у, следећи LTI launch синхронизује промену.

#### Висина iframe-а и ресајз

FastComments емитује `org.imsglobal.lti.frameResize` postMessage при сваком рендеровању нити и при променама садржаја (нов коментар, проширене повратне поруке). Brightspace слуша за ову поруку и прилагођава висину iframe-а тако да нит није исечена и да не показује унутрашњи scrollbar.

Ако iframe остане на фиксно малој висини:

- Потврдите да се курс учитава преко HTTPS. Brightspace-ов слушач postMessage-а одбацује frames са мешовитим садржајем.
- Потврдите да ниједан browser extension не блокира postMessage канал.
- За inline уградње у HTML теми, околни HTML не сме да омотава iframe у контејнер са фиксном висином. Уклоните било какav inline `style="height: ..."` са родитељског елемента.

#### Brightspace-специфични проблеми

**Алат се не приказује у Add Existing picker-у.** Deployment није омогућен за организациону јединицу овог курса. Администратор треба да дода организациону јединицу (или родитеља) у списак **Org Units** за deployment. Само регистрација алата није довољна; deployment одређује који курсеви виде алат.

**`deployment_id` mismatch on launch.** FastComments TOFU-пини први `deployment_id` који види за регистрацију. Ако администратор избрише оригинални deployment и креира нови, покретања са новог deployment-а се одбијају са грешком о неслагању deployment-а. Решење је поновно регистровање FastComments-а (генеришите нови registration URL и покрените Dynamic Registration поново); стара конфигурациона ставка се замењује.

**Алат се покреће али показује "Invalid LTI launch".** Курс је у другом tenant/org структури него што покрива deployment, или је deployment онемогућен након регистрације. Проверите поново **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > прекидач **Enabled** и списак организационих јединица за deployment.

**Имена и улоге недостају унутар FastComments-а.** Brightspace шаље LTI launch-еве са Names and Role Provisioning Services (NRPS) claims. Ако је курс надограђен са старије LTI 1.1 везе, launch може да нема `name` и `email` claims. Додајте поново FastComments тему преко **Add Existing** (немојте мигрирати стару везу) тако да launch користи LTI 1.3.

**Уградња приказује страницу за пријаву уместо аутоматског SSO.** HTML тема је уметнута као обичан `<iframe>` који показује на FastComments уместо путем **Insert Stuff** > **LTI Advantage**. Обични iframe-ови прескачу LTI launch и кориснике доводе на јавну FastComments страницу. Избришите iframe и уметните поново помоћу Insert Stuff флоуа.