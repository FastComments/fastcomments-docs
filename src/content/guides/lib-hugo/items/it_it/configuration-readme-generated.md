Tutte le opzioni del widget FastComments sono impostate in `[params.fastcomments]` in `hugo.toml`, e possono essere sovrascritte per singola pagina nel front matter in `[fastcomments]`. Precedenza, dalla più bassa alla più alta: parametri del sito, front matter della pagina, parametri dello shortcode.

```toml
# hugo.toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  hasDarkBackground = true
  voteStyle = 1
  enableSearch = true
```

```toml
# front matter di una pagina
+++
title = "Article"
[fastcomments]
  urlId = "article-42"
  collapseReplies = true
+++
```

Quando non vengono forniti né `url` né `urlId`, `url` assume come valore predefinito il permalink della pagina, così che i thread di commenti restino legati a un URL stabile.

### Residenza dei dati nell'UE

I clienti UE impostano `region = "eu"` per instradare il widget verso `cdn-eu.fastcomments.com`:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Nota sull'uso delle maiuscole nelle chiavi

Hugo converte in minuscolo ogni chiave in `hugo.toml` e nel front matter, ma i widget FastComments richiedono chiavi in camelCase (`tenantId`, `hasDarkBackground`). Questo componente ripristina automaticamente la corretta capitalizzazione per ogni opzione di primo livello nota, quindi scrivi le opzioni nella loro normale forma camelCase. Le chiavi annidate all'interno di un valore oggetto (ad esempio le chiavi di una mappa `translations`, o i campi di `pageReactConfig`) non vengono ripristinate. Configurale invece tramite l'interfaccia di personalizzazione della dashboard FastComments.