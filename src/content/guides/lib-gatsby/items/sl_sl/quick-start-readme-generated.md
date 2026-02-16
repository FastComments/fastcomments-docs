1.  **Oglejte si v akciji.**

    Pojdite v imenik vaše nove strani in jo zaženite.

    ```shell
    cd fastcomments-gatsbyjs-example/
    npm install
    gatsby develop
    ```

2.  **Dodajte FastComments na vašo Gatsby stran.**

    Oglejte si src/pages/index.js. Ta ena vrstica prikazuje, kako instanciramo pripomoček:

    ```
    return <FastCommentsCommentWidget tenantId="demo" />;
    ```