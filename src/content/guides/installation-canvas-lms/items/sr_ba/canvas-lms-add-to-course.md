#### Kako se komentari pojavljuju u vašim kursevima

Kада је LTI интеграција омогућена и додатак (External App) инсталиран, FastComments ради аутоматски на основу поставки које сте конфигурисали:

#### Assignment View

Ако је поставка **Assignment View** омогућена, коментари се аутоматски појављују испод сваког задатка у курсу. Студенти и инструктори виде нитовани одељак за коментаре када прегледају задатак — није потребна додатна подешавања по задатку.

Сваки задатак добија своју посебну нит коментара.

#### Rich Content Editor Button

Ако је поставка **Editor Button** омогућена, инструктори могу уградити FastComments у било који садржај који користи Rich Content Editor:

1. Уредите **Page**, **Quiz**, или **Announcement**.
2. На алатној траци Rich Content Editor-а, кликните на дугме **FastComments**.
3. FastComments се аутоматски уграђује у садржај.
4. Сачувајте страницу.

Када студенти прегледају страницу, уграђени FastComments виџет се учитава са нити коментара јединственом за ту страницу.

#### Automatic SSO

У оба начина поставке, студенти се аутоматски пријављују преко свог Canvas налога. Имена, е-поруке и аватари се синхронизују преко LTI launch-а, није потребна посебна пријава.

#### Lock Down Public Access (Recommended)

По подразумевању, подаци коментара у FastComments-у су јавни и читају се. Свако ко може погодити URL нити или API endpoint може видјети њене коментаре, чак и изван Canvas-а. За дискусије у курсу, скоро сигурно желите ограничити преглед само на уписане студенте.

Отворите вашу <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget customization page</a> и креирајте правило са омогућеном опцијом **Require SSO To View Comments**, затим подесите ниво сигурности на **Secure SSO** тако да се нити могу учитавати само кроз потписано LTI покретање.

Погледајте [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) за потпуни водич, укључујући како да ограничи те правило на један домен или страницу.