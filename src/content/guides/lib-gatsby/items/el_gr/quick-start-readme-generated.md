1.  **Δείτε σε δράση.**

    Μεταβείτε στον κατάλογο του νέου ιστότοπού σας και ξεκινήστε τον.

    ```shell
    cd fastcomments-gatsbyjs-example/
    npm install
    gatsby develop
    ```

2.  **Προσθέστε το FastComments στον ιστότοπό σας Gatsby.**

    Δείτε το src/pages/index.js. Αυτή η μία γραμμή είναι ο τρόπος με τον οποίο αρχικοποιούμε το widget:

    ```
    return <FastCommentsCommentWidget tenantId="demo" />;
    ```