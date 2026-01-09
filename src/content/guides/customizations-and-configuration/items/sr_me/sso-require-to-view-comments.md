FastComments SSO (<a href="#sso">детаљи овдје</a>) омогућава вашим корисницима да коментаришу без потребе да се пријављују на другу платформу.

Међутим, само то не обезбјеђује ваше ните коментара, пошто су подаци о коментарима по подразумеваној поставци јавне информације - било ко ко може видјети страницу може видјети и коментаре.

Промјеном поставке можемо ограничити преузимање коментара осим ако их не преузима администратор или важећи SSO корисник.

#### Подешавање без кода

Можемо спречити преглед и интеракцију са нашим нитима коментара, када је SSO подешен, креирањем <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">правила прилагођавања</a>.

When doing so, search for SSO, and you will find this option:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; title='Require SSO To View Comments' app-screenshot-end]

Омогућите је и сачувајте правило прилагођавања.

#### Заштитите само одређени домен или страницу

Да бисте заштитили само одређени домен или страницу, једноставно ћемо конфигурисати правило прилагођавања да то уради.

На врху UI за прилагођавање наћи ћемо два уноса, Domain and URL ID.

Да бисте само заштитили одређени домен, унесите тај домен у поље "domain".

Да бисте заштитили одређену страницу, унесите URL странице у поље "URL ID". Ако имате прилагођену интеграцију са FastComments, можете овдје унети врсту ID-а уместо URL-а.

#### Нивои безбједности

When requiring SSO, you'll want to decide if you require Simple SSO or Secure SSO. If you require Simple SSO, then both are allowed, but if you require Secure SSO then
the content must be fetched with a Secure SSO payload hashed with your API key in order to be viewed.

Опција нивоа сигурности појавиће се када изаберете "Require SSO To View Comments".

#### Заштита изван читања

Омогућавање ове опције заштитиће страницу или домен од коментара, осим ако корисник није пријављен преко SSO.

#### Напомене

Сви корисници који су креирали коментаре прије ваше SSO интеграције неће моћи да их виде, осим ако се не пријаве преко ваше SSO интеграције.