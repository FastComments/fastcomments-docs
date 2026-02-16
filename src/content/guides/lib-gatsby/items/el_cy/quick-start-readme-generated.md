1.  **Δείτε το σε δράση.**

    Περιηγηθείτε στον κατάλογο του νέου σας ιστότοπου και ξεκινήστε τον.

    ```shell
    cd fastcomments-gatsbyjs-example/
    npm install
    gatsby develop
    ```

2.  **Προσθέστε το FastComments στον ιστότοπό σας Gatsby.**

    Ρίξτε μια ματιά στο src/pages/index.js. Αυτή η μία γραμμή δείχνει πώς αρχικοποιούμε το widget:

    ```
    return <FastCommentsCommentWidget tenantId="demo" />;
    ```