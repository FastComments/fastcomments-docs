In FastComments noemen we de artikel-id, of de pagina waaraan de reacties gekoppeld zijn, de URL ID, omdat dit een URL of een ID kan zijn.
Definieer de URL ID op de volgende manier. De component houdt wijzigingen in het config-object in de gaten en zal opnieuw laden, dus u kunt de URL ID bijwerken.

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### Accountregio (LET OP: EU-klanten)

Als uw account zich in de EU bevindt, stel in de widgetconfiguratie `region = 'eu'` in, bijvoorbeeld:

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

Anders hoeft u `region` niet te definiëren.