1.  **실행 예 보기.**

    새 사이트 디렉터리로 이동하여 시작하세요.

    ```shell
    cd fastcomments-gatsbyjs-example/
    npm install
    gatsby develop
    ```

2.  **Gatsby 사이트에 FastComments 추가하기.**

    src/pages/index.js를 살펴보세요. 이 한 줄이 위젯을 인스턴스화하는 방법입니다:

    ```
    return <FastCommentsCommentWidget tenantId="demo" />;
    ```