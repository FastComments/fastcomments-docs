У FastComments ми називаємо id статті або сторінки, до якої прив'язані коментарі, URL ID, оскільки це може бути url або ID.
Визначте URL ID наступним чином. Компонент стежить за змінами в об'єкті config і перезавантажиться, тож ви можете просто оновити налаштування "url" та "urlId".

Дивіться повний робочий приклад [тут](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts).

Запустіть приклад пагінації за допомогою:

```
npm run serve-pagination
```

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id'}" />
```

### Регіон облікового запису (УВАГА: клієнти з ЄС)

Якщо ваш обліковий запис розміщено в ЄС, встановіть `region = 'eu'` у конфігурації віджета, наприклад:

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id', region: 'eu'}" />
```

В іншому випадку вам не потрібно визначати `region`.