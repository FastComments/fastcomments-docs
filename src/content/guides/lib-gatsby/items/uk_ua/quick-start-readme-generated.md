1.  **Побачити в дії.**

    Перейдіть у каталог вашого нового сайту та запустіть його.

    ```shell
    cd fastcomments-gatsbyjs-example/
    npm install
    gatsby develop
    ```

2.  **Додайте FastComments до вашого сайту Gatsby.**

    Перегляньте src/pages/index.js. Цей один рядок показує, як ми створюємо екземпляр віджета:

    ```
    return <FastCommentsCommentWidget tenantId="demo" />;
    ```