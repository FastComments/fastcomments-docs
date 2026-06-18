Tutte le opzioni del widget FastComments sono impostate sotto `[params.fastcomments]` in `hugo.toml`, e possono essere sovrascritte per pagina nel front matter sotto `[fastcomments]`. Precedenza, dalla più bassa alla più alta: parametri del sito, front matter della pagina, parametri dello shortcode.

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

When neither `url` nor `urlId` is provided, `url` defaults to the page's permalink so comment threads stay tied to a stable URL.

### Residenza dei dati UE

I clienti UE impostano `region = "eu"` per instradare il widget verso `cdn-eu.fastcomments.com`:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Nota sulla capitalizzazione delle chiavi

Hugo converte in minuscolo ogni key in `hugo.toml` e nel front matter, ma i widget FastComments richiedono chiavi in camelCase (`tenantId`, `hasDarkBackground`). Questo componente ripristina automaticamente la corretta capitalizzazione per ogni opzione di primo livello conosciuta, quindi scrivi le opzioni nella loro normale forma camelCase. Le chiavi annidate all'interno di un valore oggetto (ad esempio le chiavi di una mappa `translations`, o i campi di `pageReactConfig`) non vengono ripristinate. Configura quelle tramite l'interfaccia di personalizzazione della dashboard FastComments.