In FastComments, we refer to the article ID — or the page the comments are tied to — as the URL ID, since it can be a URL or an ID.
Define the URL ID as follows. The component watches for changes in the config object and will reload, so you can update the URL ID.

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### Account Region (ATTENTION: EU Customers)

If your account is located in the EU, set `region = 'eu'` in the widget configuration, for example:

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

Otherwise, you don't have to define `region`.