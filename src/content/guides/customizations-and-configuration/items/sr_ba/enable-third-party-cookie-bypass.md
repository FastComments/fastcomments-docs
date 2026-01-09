[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

За аутентификацију, FastComments зависи од омогућених колачића трећих страна у вашем претраживачу. Без њих, корисници ће увек морати
да оставе своју е-пошту да би коментарисали (осим ако поље за унос е-поште није скривено), и њихови коментари ће увек бити приказани као неверификовани (по подразумеваној поставци).

Да бисте ово заобишли, можете омогућити обилазак колачића трећих страна. 

Када је ова опција омогућена, приказаће мали искачући прозор који показује поруку да се корисник пријављује. Овај искачући прозор
се појављује кад год корисник интерагује са видгетом за коментаре; на пример, ако остави коментар.

Ово можемо урадити у коду тако што ћемо поставити флаг **enableThirdPartyCookieBypass** на true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Enabling Third-Party Cookie Bypass'; code-example-end]

Ово такође можемо подесити преко интерфејса за прилагођавање видгета, у секцији `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; title='Enabling Third-Party Cookie Bypass' app-screenshot-end]

---