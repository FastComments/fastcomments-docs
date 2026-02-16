---
1.  **Ver en acción.**

    Navega al directorio de tu nuevo sitio e inícialo.

    ```shell
    cd fastcomments-gatsbyjs-example/
    npm install
    gatsby develop
    ```

2.  **Añade FastComments a tu sitio Gatsby.**

    Echa un vistazo a src/pages/index.js. Esta única línea es cómo instanciamos el widget:

    ```
    return <FastCommentsCommentWidget tenantId="demo" />;
    ```
---