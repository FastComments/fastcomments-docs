1.  **Посмотреть в действии.**

    Перейдите в каталог вашего нового сайта и запустите его.

    ```shell
    cd fastcomments-gatsbyjs-example/
    npm install
    gatsby develop
    ```

2.  **Добавьте FastComments на свой сайт Gatsby.**

    Откройте src/pages/index.js. Эта одна строка показывает, как мы инициализируем виджет:

    ```
    return <FastCommentsCommentWidget tenantId="demo" />;
    ```