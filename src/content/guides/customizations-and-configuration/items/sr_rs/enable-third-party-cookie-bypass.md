---
[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

За аутентификацију, FastComments зависи од омогућених колачића трећих страна у вашем прегледачу. Без њих, корисници ће увек морати
да оставе своју е‑пошту за коментарисање (осим ако је поље за унос е‑поште скривено), и њихови коментари ће увек бити приказани као непотврђени (подразумевано).

Да би заобиšli ово, можете омогућити заобилажење колачића трећих страна.

Када је ово подешавање омогућено, то ће изазвати мали искачући прозор који приказује поруку да се корисник пријављује. Овај искачући прозор се приказује кад год корисник интерагује са виџетом за коментаре; на пример, ако остави коментар.

Ово можемо урадити у коду постављањем заставице **enableThirdPartyCookieBypass** на true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Омогућавање заобилажења колачића трећих страна'; code-example-end]

Ово такође можемо подесити преко корисничког интерфејса за прилагођавање виџета, под `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; alt='Страница за прилагођавање виџета са означеним пољем за омогућавање искачућег прозора за колачиће трећих страна'; title='Омогућавање заобилажења колачића трећих страна' app-screenshot-end]

---