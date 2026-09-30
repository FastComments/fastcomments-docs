Ако премештате ваш сајт са WordPress‑а и желите FastComments на новом сајту, не треба вам WordPress додатак. Извезите ваше коментаре из WordPress‑а, а затим отпремите датотеку на [Страна за увоз](https://fastcomments.com/auth/my-account/manage-data/import) у FastComments контролној табли.

Подржавамо два WordPress формата за извоз.

### WordPress XML (Препоручено)

Ово је датотека из уграђеног WordPress извоза, тако да није потребан додатни додатак.

1. У вашој WordPress администраторској панелу, идите на `Tools -> Export`.
2. Изаберите `All content` и кликните `Download Export File`.
3. На FastComments [Страна за увоз](https://fastcomments.com/auth/my-account/manage-data/import), изаберите `WordPress (.xml)` и отпремите датотеку.

Сваки коментар је везан за URL поста на ком је остављен, што је већ укључено у датотеку.

Увоз задржава име аутора, имејл, веб сајт, датум, садржај, нити одговора и статус одобрења коментара. Аватари коментатора се преузимају са Gravatar‑а. Гласови нису део овог формата.

### WordPress CSV

Ово је датотека из [WebToffee‑овог WordPress Comments Import & Export plugina](https://wordpress.org/plugins/comments-import-export-woocommerce/).

1. Инсталирајте plugin у вашој WordPress администраторској панелу и извезите коментаре као CSV.
2. Замените сваку вредност `comment_post_ID` URL‑ом поста.
3. На FastComments [Страна за увоз](https://fastcomments.com/auth/my-account/manage-data/import), изаберите `WordPress (.csv)` и отпремите датотеку.

Сваки коментар је везан за колону `comment_post_ID`. WordPress попу ова колона садржи ID поста, а ваш нови сајт нема WordPress ID‑ове, па корак 2 заменjuje их URL‑ом.

Увоз задржава име аутора, имејл, веб сајт, датум, садржај, нити одговора и статус одобрења коментара. Аватари коментатора се преузимају са Gravatar‑а. Такође задржава WordPress‑ов спам флаг, као и wpDiscuz лајкове и дислајкове када су они присутни у датотеци.

### Усклађивање коментара са вашим новим страницама

Ако ваш нови сајт задржи исте URL‑ове као ваш WordPress сајт, коментари ће се појавити на одговарајућим страницама без додатних подешавања.

Ако се домен промени, покрените [Domain Migration tool](/guide-migrations.html#migrating-domains) након увоза. Ако се појединачни URL‑ови страница промене, можете [мигрирати сваку страницу](/guide-migrations.html#migrating-pages) са старог URL‑а на нови.

За масовне миграције страница, као што је уклањање домена из вредности коју прослеђујете пољу [urlId](/guide-customizations-and-configuration.html#url-id) у widget‑у за коментаре, [отворите тикет за подршку](https://fastcomments.com/auth/my-account/help) и ми ћемо то урадити за вас.

### Пре него што пређете

Можете покретати увоз колико год желите. Поновни увоз исте датотеке [не прави дупликате](/guide-migrations.html#importing-data), тако да можете једном увезти да тестирате нови сајт, а затим поново увезти најновије коментаре непосредно пре преласка.

За датотеке извоза веће од 1 GB, [обратите се подршци](https://fastcomments.com/auth/my-account/help).

Да бисте додали FastComments на ваш нови сајт, погледајте [Installation guide](/guide-installation.html).