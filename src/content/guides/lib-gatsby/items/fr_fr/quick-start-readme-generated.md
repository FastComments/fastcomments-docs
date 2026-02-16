---
1.  **Voir en action.**

    Placez-vous dans le répertoire de votre nouveau site et lancez-le.

    ```shell
    cd fastcomments-gatsbyjs-example/
    npm install
    gatsby develop
    ```

2.  **Ajoutez FastComments à votre site Gatsby.**

    Regardez le fichier src/pages/index.js. Cette ligne montre comment nous instancions le widget :

    ```
    return <FastCommentsCommentWidget tenantId="demo" />;
    ```
---