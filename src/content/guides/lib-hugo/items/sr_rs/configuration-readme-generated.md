Све FastComments виџет опције се постављају под `[params.fastcomments]` у `hugo.toml`, и могу се преписати по страници у front matter‑у под `[fastcomments]`. Приоритет, од најниже до највише: параметри сајта, front matter странице, параметри кратког кода.

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

Када ни `url` ни `urlId` нису наведени, `url` подразумевано добија permalink странице тако да теме коментара остану везане за стабилну URL адресу.

### Резиденција података у ЕУ

Клијенти из ЕУ постављају `region = "eu"` да би усмерили виџет ка `cdn-eu.fastcomments.com`:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Напомена о великим/малим словима у кључевима

Hugo претвара сваки кључ у `hugo.toml` и front matter у мала слова, али FastComments виџети захтевају camelCase кључеве (`tenantId`, `hasDarkBackground`). Ова компонента аутоматски враћа исправно велико/мало слово за сваку познату опцију највишег нивоа, па пишите опције у њиховом уобичајеном camelCase облику. Кључеви угнеждени унутар објектних вредности (на пример кључеви мапе `translations` или поља `pageReactConfig`) се не враћају. Конфигуришите их преко FastComments контролне табле за прилагођавање уместо тога.