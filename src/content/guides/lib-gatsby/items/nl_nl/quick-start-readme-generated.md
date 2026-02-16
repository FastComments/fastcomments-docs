1.  **Bekijk het in actie.**

    Navigeer naar de map van je nieuwe site en start deze op.

    ```shell
    cd fastcomments-gatsbyjs-example/
    npm install
    gatsby develop
    ```

2.  **Voeg FastComments toe aan je Gatsby-site.**

    Bekijk src/pages/index.js. Deze ene regel toont hoe we de widget instantieren:

    ```
    return <FastCommentsCommentWidget tenantId="demo" />;
    ```