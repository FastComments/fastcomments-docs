---
1.  **Pogledajte u akciji.**

    Uđite u direktorijum vašeg novog sajta i pokrenite ga.

    ```shell
    cd fastcomments-gatsbyjs-example/
    npm install
    gatsby develop
    ```

2.  **Dodajte FastComments na vaš Gatsby sajt.**

    Pogledajte fajl src/pages/index.js. Ova jedna linija pokazuje kako instanciramo widget:

    ```
    return <FastCommentsCommentWidget tenantId="demo" />;
    ```
---