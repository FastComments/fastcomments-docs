---
Модератори се могу распоредити у групе како би модерирали различите странице или категорије садржаја.

Када модератор припада једној или више група, он ће у страници за модерирање коментара видети само коментаре из тих група.

На пример, рећи ћемо да имамо сајт који приказује видео записе по категоријама. Можда желимо различите модераторе за видео записе о мачкама, псима и папагајима, па [додајмо те групе](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

[app-screenshot-start url='/auth/my-account/moderate-comments/moderation-groups?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderation-groups'; selector = '.content'; alt='Листа група за модерацију са групама за мачке, псе и папагаје креираним за сваку категорију видеа'; title='Страница група за модерацију' app-screenshot-end]

Када додајемо модератора, сада имамо могућност да изаберемо једну или више група којима ће модератор припадати:

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; alt='Форма за додавање модератора са изборником група који се користи за доделу модератора једној или више група'; title='Додавање модератора и избор групе' app-screenshot-end]

На крају, коментари морају бити повезани са једном или више група како би их прави модератори видели.

Ово се може поставити [додавањем неких група](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) и затим навођењем одговарајућих `Moderation Group` ID‑ева у виџету за коментаре,
[као што је описано овде](/guide-customizations-and-configuration.html#moderation-group-ids).

---