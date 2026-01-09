[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

За аутентификацију, FastComments зависи од колачића трећих страна да буду омогућени у вашем прегледачу. Без њих, корисници ће увек морати да
оставе своју е-пошту да би коментарисали (осим ако поље за унос е-поште није сакривено), а њихови коментари ће по подразумеваној поставци увек бити означени као неверификовани.

Да бисте то заобишли, можете омогућити заобилажење трећих колачића. 

Када је ова поставка омогућена, појавиће се мали искачући прозор који приказује поруку да се корисник пријављује. Овај искачући прозор
се појављује кад год корисник интерагује са видгетом за коментаре; на пример, ако оставе коментар.

Ово можемо урадити у коду постављањем флага **enableThirdPartyCookieBypass** на true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Enabling Third-Party Cookie Bypass'; code-example-end]

Ово такође можемо подесити преко корисничког интерфејса за прилагођавање видгета, у одељку `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; title='Enabling Third-Party Cookie Bypass' app-screenshot-end]

---