Bij FastComments noemen we de artikel-id, of de pagina waaraan de reacties gekoppeld zijn, de URL ID omdat het een url of een ID kan zijn.
Definieer de URL ID op de volgende manier. De component let op wijzigingen in het config-object en zal opnieuw laden, dus je kunt gewoon de instellingen "url" en "urlId" bijwerken.

Zie een volledig werkend voorbeeld [hier](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts).

Voer het voorbeeld voor paginering uit via:

```
npm run serve-pagination
```

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id'}" />
```

### Accountregio (LET OP: EU-klanten)

Als je account zich in de EU bevindt, stel dan `region = 'eu'` in de widgetconfiguratie in, bijvoorbeeld:

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id', region: 'eu'}" />
```

Anders hoef je `region` niet te definiëren.