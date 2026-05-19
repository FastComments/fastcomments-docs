Тази страница описва добавянето на FastComments към курс в Brightspace след като администраторът е регистрирал инструмента и е създал deployment. Ако инструментът все още не е регистриран, първо вижте D2L регистрационното ръководство.

<div class="screenshot white-bg">
    <div class="title">FastComments вграден като тема на модул в Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments работи в рамките на Brightspace модул, показва нишкови коментари и селектор за @-споменаване" />
</div>

Brightspace предоставя два начина за създаване на съдържание: **Classic Content** и **New Content Experience** (наричано още **Lessons**). И в двата случая FastComments е достъпен, но пътеките в менюто се различават. Във всеки от разделите по-долу е посочено и за двете среда там, където има разлики.

#### Locate the FastComments Tool

Инструментът FastComments се появява на две места в редактора на съдържание на курса:

1. Activity picker-а, достъпен от бутона **Add Existing** на модул/юнит (в по-стари версии на Brightspace е надписан **Add Existing Activities**). FastComments се показва директно в picker-а в текущите версии на Brightspace; в по-стари версии е вложен под подменюто **External Learning Tools**. И по двата пътя FastComments се добавя като самостоятелна тема.
2. Диалогът **Insert Stuff** в HTML редактора, под **LTI Advantage**. Това вгражда FastComments инлайн в HTML тема чрез deep linking потока на LTI.

Ако FastComments не се появява в нито един от picker-ите, deployment-ът не е активиран за org unit-а, който съдържа курса. Помолете администратора на Brightspace да отвори **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, да отвори deployment-а и да добави org unit-а на курса (или родителски org unit) под **Org Units**.

#### Add FastComments as a Topic in a Module

Classic Content:

1. Отворете курса и кликнете **Content** в навигацията.
2. Изберете модула, който трябва да съдържа обсъждането (или създайте такъв чрез **Add a module**).
3. Кликнете **Add Existing** (по-стари версии на Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. В picker-а кликнете **FastComments**. Brightspace създава тема в модула и ви връща към изгледа на съдържанието.
5. Кликнете новата тема. Преименувайте я на нещо описателно като `FastComments Discussion` с помощта на inline редактора за заглавие.

New Content Experience (Lessons):

1. Отворете курса и кликнете **Content**.
2. Отворете unit-а и lesson-а, който трябва да съдържа обсъждането.
3. Кликнете **Add** > **Existing Activity** и изберете **FastComments** (по-стари версии на Brightspace: вложено под **External Learning Tools**).
4. Активността се добавя към lesson-а.
5. Кликнете заглавието на активността, за да я преименувате.

Първия път, когато някой потребител (инструктор или студент) отвори темата, FastComments инициализира нишката за този resource link. Нишката е свързана с resource link ID, така че преименуването или преместването на темата не променя коя нишка се зарежда.

#### Embed FastComments Inline in an HTML Topic

Използвайте този поток, когато искате коментарите да се появяват под текст, видео или друго съдържание в същата страница на темата, вместо като отделна тема.

1. Отворете или създайте HTML тема в модула/lesson-а.
2. Кликнете **Edit HTML**, за да отворите HTML редактора на Brightspace.
3. Поставете курсора на мястото, където трябва да се появи нишката с коментари.
4. Кликнете бутона **Insert Stuff** (иконка пъзел в тулбара на редактора).
5. В диалога Insert Stuff превъртете до **LTI Advantage** и кликнете **FastComments**.
6. FastComments отваря deep linking picker. Потвърдете поставянето (по подразбиране опциите работят за content discussions); кликнете **Insert** или **Continue**.
7. Brightspace се връща в HTML редактора с плейсхолдер блок, представляващ LTI launch. Кликнете **Save and Close** на темата.

Когато темата се зареди, Brightspace замества плейсхолдера с iframe, който автоматично стартира FastComments чрез LTI. Студентите виждат нишката за обсъждане инлайн.

Една HTML тема може да съдържа множество deep-linked FastComments вграждания. Всяко вграждане получава своя собствена нишка, защото всяка deep link-а генерира отделен resource link ID.

#### Module Topic vs Inline Quicklink

Изберете подхода с **module topic** когато:

- Обсъждането е основната активност за този етап от модула.
- Искате темата да се появи в съдържанието на Brightspace, в проследяването на завършване и в Class Progress.

Изберете подхода с **inline embed** когато:

- Коментарите трябва да са под друго съдържание на същата страница.
- Не искате отделен елемент, който да се проследява за завършване в съдържанието.

#### Visibility, Draft, and Release Conditions

Нова тема на FastComments е видима за студентите по подразбиране. За да я скриете докато я настройвате:

1. В редактора на съдържание кликнете заглавието на темата (Classic) или менюто с три точки на активността (New Content Experience).
2. Задайте статус на **Draft** (Classic) или изключете **Visibility** (New Content Experience).

Draft темите са невидими за студентите. Инструкторите и асистентите все още ги виждат с етикет "Draft".

За да ограничите темата до конкретна група или секция:

1. Отворете темата.
2. Кликнете менюто на заглавието > **Edit Properties In-place** (Classic) или **Edit** > **Restrictions** (New Content Experience).
3. Под **Release Conditions** кликнете **Create**.
4. Изберете **Group enrollment** или **Section enrollment**, посочете групата/секцията и запазете.

Release conditions работят заедно със собственото мапиране на роли на FastComments. Студенти, които не могат да виждат темата, не получават LTI launch.

#### What Students See on First Launch

Когато студент кликне темата (или зареди HTML тема с вграждане):

1. Brightspace извършва LTI 1.3 launch на заден план.
2. FastComments получава името на студента, имейла, URL за аватар и LMS ролята и го логва автоматично. Няма подкана за вход в FastComments.
3. Нишката за коментари за този resource link се рендерира вътре в iframe-а на Brightspace.

Мапиране на роли при launch:

- Brightspace `Administrator` става FastComments администратор (admin) за нишката (пълен достъп до модерация, изтриване, бан и конфигурация).
- Brightspace `Instructor` става FastComments модератор (moderator) (pin, hide, delete, ban).
- Всички останали роли (`Learner`, `TeachingAssistant`, и т.н.) стават стандартни коментатори.

Коментарите се приписват на акаунта на студента в Brightspace. Ако студентът промени името или аватара си в Brightspace, следващият LTI launch синхронизира промяната.

#### Iframe Height and Resize

FastComments изпраща postMessage `org.imsglobal.lti.frameResize` при всяко рендериране на нишката и при промени в съдържанието (нов коментар, отваряне на отговори). Brightspace слуша за това съобщение и наглася височината на iframe-а така, че нишката да не бъде отрязана и да не се появява вътрешен скролбар.

Ако iframe остава с фиксирана ниска височина:

- Потвърдете, че курсът се зарежда през HTTPS. Слушателят за postMessage на Brightspace отхвърля фреймове със смесено съдържание.
- Потвърдете, че никое разширение на браузъра не блокира postMessage канала.
- За инлайн вграждания в HTML тема, обкръжаващият HTML не трябва да обвива iframe-а в контейнер с фиксирана височина. Премахнете всякакъв inline `style="height: ..."` от родителския елемент.

#### Brightspace-Specific Gotchas

**Tool not showing in the Add Existing picker.** Deployment-ът не е активиран за org unit-а на този курс. Администратор трябва да добави org unit-а (или родителски) в списъка **Org Units** на deployment-а. Самата регистрация на инструмента не е достатъчна; deployment-ът определя за кои курсове инструментът е видим.

**`deployment_id` mismatch on launch.** FastComments фиксира (TOFU) първия `deployment_id`, който види за една регистрация. Ако администраторът изтрие оригиналния deployment и създаде нов, launch-овете от новия deployment се отхвърлят с грешка за несъответствие на deployment-а. Решението е да се регистрира FastComments отново (генерирайте нов registration URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">get it here</a>) и стартирайте Dynamic Registration отново); старата конфигурация ще бъде заменена.

**Tool launches but shows "Invalid LTI launch".** Курсът е в различна tenant/org структура от тази, която покрива deployment-а, или deployment-ът е бил деактивиран след регистрацията. Проверете отново **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > превключвателя **Enabled** и списъка с org unit-и в deployment-а.

**Names and roles missing inside FastComments.** Brightspace изпраща LTI launch-ове с Names and Role Provisioning Services (NRPS) claims. Ако курсът е бил обновен от по-стар LTI 1.1 линк, launch-ът може да няма `name` и `email` claims. Пре-добавете FastComments темата чрез **Add Existing** (не мигрирайте стария линк), така че launch-ът да използва LTI 1.3.

**Embed shows a login screen instead of auto-SSO.** HTML темата е вмъкната като обикновен `<iframe>` насочен към FastComments, вместо чрез **Insert Stuff** > **LTI Advantage**. Обикновените iframe-ове пропускат LTI launch-а и пренасочват потребителите към публичната страница на FastComments. Изтрийте iframe-а и го вмъкнете отново чрез Insert Stuff потока.