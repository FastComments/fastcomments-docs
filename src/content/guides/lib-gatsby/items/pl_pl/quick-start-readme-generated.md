1.  **Zobacz w akcji.**

    Przejdź do katalogu swojej nowej witryny i uruchom ją.

    ```shell
    cd fastcomments-gatsbyjs-example/
    npm install
    gatsby develop
    ```

2.  **Dodaj FastComments do swojej witryny Gatsby.**

    Zajrzyj do src/pages/index.js. Ta jedna linia pokazuje, jak inicjalizujemy widżet:

    ```
    return <FastCommentsCommentWidget tenantId="demo" />;
    ```