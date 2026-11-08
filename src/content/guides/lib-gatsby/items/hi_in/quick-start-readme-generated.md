1.  **एक्शन में देखें।**

    अपनी नई साइट की डायरेक्टरी में जाएँ और इसे स्टार्ट करें।

    ```shell
    cd fastcomments-gatsbyjs-example/
    npm install
    gatsby develop
    ```

2.  **अपनी Gatsby साइट में FastComments जोड़ें।**

    src/pages/index.js को देखें। यह एक लाइन ही है जिससे हम विजेट को इंस्टैंटिएट करते हैं:

    ```
    return <FastCommentsCommentWidget tenantId="demo" />;
    ```