1.  **Uygulamada Görün.**

    Yeni sitenizin dizinine gidin ve başlatın.

    ```shell
    cd fastcomments-gatsbyjs-example/
    npm install
    gatsby develop
    ```

2.  **FastComments'ı Gatsby Sitenize Ekleyin.**

    src/pages/index.js dosyasına göz atın. Bu tek satır, widget'i nasıl örneklediğimizdir:

    ```
    return <FastCommentsCommentWidget tenantId="demo" />;
    ```