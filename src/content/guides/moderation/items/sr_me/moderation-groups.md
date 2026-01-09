Модератори могу бити стављени у групе како би модерирали различите странице или категорије садржаја.

Када модератор припада једној или више група, на страници за модерирање коментара видеће само коментаре из тих група.

На пример, рецимо да водимо сајт који приказује видео снимке по категоријама. Можда ћемо желети различите модераторе за видео снимке о мачкама, псима и папагајима, па [додајмо те групе](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

[app-screenshot-start url='/auth/my-account/moderate-comments/moderation-groups?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderation-groups'; selector = '.content'; title='The Moderation Groups Page' app-screenshot-end]

Када додамо модератора, сада имамо опцију да одаберемо једну или више група којима ће модератор припадати:

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; title='Adding A Moderator and Selecting a Group' app-screenshot-end]

На крају, коментари морају бити повезани са једном или више група како би их видели одговарајући модератори.

Ово се може подесити тако што ћете [додати неке групе](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) и потом одредити одговарајуће `Moderation Group` ids у видгету за коментаре,
[као што је објашњено овде](/guide-customizations-and-configuration.html#moderation-group-ids).

---