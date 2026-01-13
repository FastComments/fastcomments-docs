У FastComments ми називаємо ідентифікатор статті або сторінки, до якої прив'язані коментарі, URL ID, оскільки це може бути URL або ID.
Задайте URL ID таким чином. Компонент відстежує зміни об'єкта config і перезавантажиться, тому ви можете оновлювати URL ID.

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### Регіон акаунту (УВАГА: клієнти з ЄС)

Якщо ваш акаунт розташовано в ЄС, встановіть `region = 'eu'` у конфігурації віджета, наприклад:

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

В іншому випадку вам не потрібно визначати `region`.