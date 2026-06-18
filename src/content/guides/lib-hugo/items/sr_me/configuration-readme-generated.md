Све опције FastComments виџета се подешавају под `[params.fastcomments]` у `hugo.toml`, и могу се по страници прекрити у front matter под `[fastcomments]`. Приоритет, од најнижег до највишег: параметри сајта, front matter странице, параметри шорткода.

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

Кад ни `url` ни `urlId` нису обезбеђени, `url` подразумевано постаје permalink странице тако да нит коментара остане везана за стабилан URL.

### Резиденција података у ЕУ

Клијенти из ЕУ подешавају `region = "eu"` да усмере виџет ка `cdn-eu.fastcomments.com`:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Напомена о регистру кључева

Hugo претвара сваки кључ у `hugo.toml` и у front matter у мала слова, али FastComments виџети захтијевају camelCase кључеве (`tenantId`, `hasDarkBackground`). Ова компонента аутоматски враћа тачан регистар за сваку познату опцију највишег нивоа, па упишите опције у њиховом уобичајеном camelCase облику. Кључеви угнеждени унутар вредности објекта (на примјер кључеви мапе `translations`, или поља `pageReactConfig`) се не враћају. Конфигуришите те кроз кориснички интерфејс за прилагођавање у FastComments контролној табли уместо тога.