V FastComments imenujemo ID članka oziroma strani, na katero so komentarji pripeti, URL ID, saj je lahko to URL ali ID.
URL ID določite na naslednji način. Komponenta spremlja spremembe v objektu config in se bo ponovno naložila, tako da lahko posodobite URL ID.

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### Regija računa (POZOR: stranke v EU)

Če se vaš račun nahaja v EU, nastavite `region = 'eu'` v konfiguraciji vtičnika, na primer:

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

V nasprotnem primeru vam ni treba določiti `region`.