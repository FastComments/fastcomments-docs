Once an administrator has registered FastComments as an LTI 1.3 Advantage tool and approved the institution policies, instructors add it to courses through the standard Blackboard placement points. The exact steps differ between Ultra Course View and Original Course View, so both are covered below.

#### Ultra Course View

Ultra Course View је подразумеван у Blackboard Learn SaaS од 2026. године.

1. Отворите курс и идите на страницу **Course Content**.
2. Померите курсор или додирните место где желите да нит коментара буде у оквиру наслова и кликните љубичасти **+** (Add content) тастер.
3. Изаберите **Content Market**. Панел Content Market приказује све одобрене LTI алате и Building Block поставке за вашу институцију.
4. Пронађите плочицу **FastComments** и кликните на њу. Blackboard креира садржајну ставку на позицији где сте отворили **+** мени.
5. Ставка се подразумевано појављује у оквиру као унос „Visible to students“ за инструкторе који имају опцију **Hide from students** искључену као свој лични подразумевани. Ако вам је подразумевано **Hidden**, ставка се креира као скривена и ви укључите селектор видљивости на реду ставке када будете спремни.
6. Да преименујете ставку, кликните на наслов у оквиру и унесите нову ознаку. Наслов који студенти виде у оквиру независан је од FastComments идентификатора нити, тако да је безбедно преименовати у било које време.

Ако не видите опцију **Content Market**, ваша институција је сакрила ту поставку. Исто средство за избор можете да достигнете и преко **More tools** у истом **+** менију под групом **LTI Tools**.

#### Original Course View

Original Course View још увек подржава Learn SaaS и остаје примарно искуство за самостално хостоване Learn 9.1 сајтове на Q4 2024 CU релизе линији.

1. Отворите курс и уђите у **Content Area** (на пример, подразумевана **Information** или **Content** област у менију курса).
2. Укључите **Edit Mode** помоћу прекидача у горњем десном углу странице.
3. Кликните **Build Content** у траци акција.
4. У подменију **Learning Tools** кликните **FastComments**. Подменију Learning Tools попуњавају LTI 1.3 поставке алата након што администратор региструје алат. Ако га не видите, погледајте одељак о проблемима испод.
5. На формулару **Create FastComments** подесите:
   - **Name**: ознака коју студенти виде у области садржаја.
   - **Description**: опционални текст који се приказује изнад уграђене нити.
   - **Permit Users to View this Content**: Да/Не прекидач доступности.
   - **Track Number of Views**: омогућите ако желите појединачну статистику прегледа по ставкама у Blackboard-у. FastComments води своју анализу независно.
   - **Date and Time Restrictions**: опционалне опције **Display After** / **Display Until**.
6. Потврдите. Алат се појављује као кликабилна ставка у области садржаја.

#### Embedding Inside an Item or Document

У оба приказа курса, инструктори убацују FastComments инлајн у тело ставке, документа или било које rich-text поље преко LTI Advantage дугмета у Content Editor-у.

Ultra Course View:

1. Креирајте или уредите **Document**.
2. Кликните **Add content** унутар тела документа на месту где желите да нит буде.
3. У траци уређивача отворите мени **Insert content** и кликните **Content Market** (улазна тачка за LTI Advantage / Deep Linking).
4. Одаберите **FastComments**. FastComments враћа deep-link payload и Blackboard убацује уграђени блок у тело документа на позицији курсора.
5. Сачувајте документ. Студенти виде нит уграђену у линији када пролазе поред ње.

Original Course View:

1. Уредите било коју ставку са rich-text телом.
2. У Content Editor траци кликните иконицу **Add Content** и одаберите **Content Market** (означено као **Add Content from External Tool** у старијим Q4 2024 CU верзијама).
3. Одаберите **FastComments**. Едитор убацује плейсхолдер блок који референцира deep-linked ресурс.
4. Поднесите ставку.

Сваки deep-link embed производи своју FastComments нит, тако да ставка са два уграђена FastComments блока има два независна тока коментара.

#### Visibility, Release Conditions, and Group Restrictions

FastComments садржајне ставке се понашају као и било која друга BlackBoard садржајна ставка у погледу правила контроле приступа која се примењују на њих.

- Ultra: кликните селектор видљивости на реду (**Visible to students**, **Hidden from students**, **Conditional availability**). Conditional availability подржава временске прозоре (датум/време), правила перформанси на основу ставки у дневнику оценa (gradebook), и правила чланства заснована на групама курса.
- Original: отворите контекстни мени ставке и изаберите **Adaptive Release** или **Adaptive Release: Advanced** да бисте оградили алат по датуму, чланству, оцени или статусу прегледа. Користите **Set Group Availability** на ставци да ограничите приступ специфичним групама курса.

FastComments поштује одлуку Blackboard-ове капије. Ако Blackboard сакрије ставку од студента, LTI покретање се никада не дешава за тог студента и он се не појављује у moderator view-у.

#### Gradebook Behavior

FastComments не враћа оцене преко LTI Advantage Assignment and Grade Services. Није аутоматски креиран колона оценa за FastComments садржајне ставке.

Ако је ваш Blackboard tenant конфигурисан да аутоматски креира колону у дневнику оценa за сваку нову ставку без обзира на метаподатке о оцењивању, празна колона ће се појавити у сваком случају. Да бисте је сакрили:

- Ultra: отворите **Gradebook**, кликните на заглавље колоне, изаберите **Edit**, и искључите **Show to students** као и **Include in calculations**. Или користите **Delete** ако ваша институција дозвољава брисање колона за ненцењене ставке.
- Original: отворите **Grade Center**, кликните на стрелицу (chevron) колоне, изаберите **Hide from Users (on/off)**, и по потреби **Hide from Instructor View** у оквиру **Column Organization**.

#### What Students See

Када студент отвори FastComments ставку или скролује до уграђеног блока:

1. Blackboard покреће LTI 1.3 поруку ка FastComments. Студент је пријављен преко SSO-а користећи свој Blackboard идентитет (име, имејл, аватар, улога) без приказивања форме за пријаву.
2. Нит коментара се рендерује у iframe-у. Тхреадинг, одговори, помињања и реакције су све доступне у зависности од подешавања коментар видџета конфигурисаних у FastComments.
3. Њихови коментари се приписују њиховом Blackboard налогу. Ако студент касније уреди своје име или слику у Blackboard-у, следеће покретање ажурира FastComments профил.

Мапирање улога из Blackboard-а у FastComments:

- **System Administrator** и **Course Builder** мапирају на FastComments **admin**.
- **Instructor** и **Teaching Assistant** мапирају на FastComments **moderator**.
- **Student**, **Guest**, и **Observer** мапирају на FastComments **commenter**.

Moderator-и виде контроле модерације (pin, hide, ban, delete) инлајн на сваком коментару у нити.

#### Thread Scoping

FastComments ограничава сваку нит по **(Blackboard host, course ID, resource link ID)**. Две FastComments ставке у истом курсу производе две нити. Иста ставка копирана преко два course shell-а (на пример, путем копирања курса) производи две нити, јер Blackboard издаје нови resource link ID током копирања. Да бисте задржали дељену нит преко копија курса, користите Deep Linking са експлицитним thread URN-ом који је конфигурисан у FastComments пре покретања копије.

#### Blackboard-Specific Gotchas

**FastComments плочица недостаје у Build Content менију (Original) или Content Market (Ultra).** Администратор је одобрио алат али оставио институционалну политику која блокира релевантну поставку. Идите на **Administrator Panel** > **Integrations** > **LTI Tool Providers**, уредите FastComments унос и потврдите да су обе поставке **Course Content Tool** (Original) и **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) омогућене. Сачувајте и освежите страницу курса.

**Грешка „Tool not configured for this context“ или „Tool is not deployed“ на покретању.** Скоуп деплоја који је регистрован током динамичке регистрације не подудара се са институционалним контекстом коме курс припада. У уносу провајдера алата у Blackboard-у, проверите да ли **Deployment ID** одговара ономе што FastComments приказује на својој LTI 1.3 Configuration страници за овог tenant-а. Ако се разликују, избришите поставку и поново покрените динамичку регистрацију из свежег registration URL-а.

**Висина iframe-а изгледа фиксна или садржај се сече.** Неки Blackboard tenant-и долазе са строгим Content Security Policy који блокира подразумевани LTI iframe-resize postMessage. FastComments шаље и Canvas-стил `lti.frameResize` поруку и IMS спецификацијом `org.imsglobal.lti.frameResize` поруку да би максимизовао компатибилност, али тенант-левел CSP пребацивање блокира слушаоца у родитељском оквиру. Замолите администратора да потврди да је `*.fastcomments.com` на LTI дозволјеној листи и да никакав прилагођени CSP заглавље не уклања postMessage догађаје. Након тога ресајз ради без додатне конфигурације.

**Копирање курса дуплира нити.** Blackboard copy курса додељује нове resource link ID-јеве за LTI поставке, па копирани курсеви почињу са празним нитима. Ово је очекивано. Ако требате да копирани курс наследи оригиналну нит, поставите Deep Linking са експлицитним thread URN-ом пре копирања, или контактирајте FastComments подршку да масовно ремапују thread ID-је.

**Студент види општу Blackboard грешку при покретању.** Узрок је недостајући или застарели `email` claim. Потврдите да институционална политика за FastComments има омогућено **Role**, **Name**, и **Email Address** под **User Fields to Send**. Сачувајте, затим покрените поново у свежеј сесији прегледача.