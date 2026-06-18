Све FastComments опције видгета подешавају се у `[params.fastcomments]` у `hugo.toml`, и могу се преписати по страници у front matter под `[fastcomments]`. Приоритет, од најнижег до највишег: параметри сајта, front matter странице, параметри шорткода.

```toml
# hugo.toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  hasDarkBackground = true
  voteStyle = 1
  enableSearch = true
```

```toml
# a page's front matter
+++
title = "Article"
[fastcomments]
  urlId = "article-42"
  collapseReplies = true
+++
```

Када нису обезбеђени ни `url` ни `urlId`, `url` подразумевано користи permalink странице тако да нити коментара остану везане за стабилан URL.

### Резиденција података у ЕУ

Купци из ЕУ постављају `region = "eu"` да би усмерили видгет на `cdn-eu.fastcomments.com`:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Напомена о регистру кључева

Hugo претвара у мала слова сваки кључ у `hugo.toml` и front matter, али FastComments видгети захтијевају кључеве у camelCase формату (`tenantId`, `hasDarkBackground`). Ова компонента аутоматски враћа исправан регистар за сваку познату опцију на највишем нивоу, па опције пишите у њиховом уобичајеном camelCase облику. Кључеви угнеждени у вредност објекта (на пример кључеви у мапи `translations`, или поља `pageReactConfig`) се не враћају. Умјесто тога конфигуришите их преко корисничког интерфејса за прилагођавање (customization UI) на контролној табли FastComments.