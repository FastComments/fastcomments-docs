Moderatorima se mogu додијелити групе за модерирање различитих страница или категорија садржаја.

Када модератор припада једној или више група, на страници за модерацију коментара видеће само коментаре из тих група.

На пример, рецимо да водимо сајт који приказује видео записе по категоријама. Можда желимо имати различите модераторе за видео записе о мачкама, псима и папагајима, па [додајмо те групе](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

[app-screenshot-start url='/auth/my-account/moderate-comments/moderation-groups?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderation-groups'; selector = '.content'; title='The Moderation Groups Page' app-screenshot-end]

Када додамо модератора, сада имамо опцију да одаберемо једну или више група којима ће модератор припадати:

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; title='Adding A Moderator and Selecting a Group' app-screenshot-end]

На крају, коментари морају бити повезани са једном или више група како би их одговарајући модератори видели.

Ово се може подесити [додавањем неколико група](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) и затим навођењем одговарајућих `Moderation Group` ID-ова у видгету за коментаре,
[као што је описано овде](/guide-customizations-and-configuration.html#moderation-group-ids).