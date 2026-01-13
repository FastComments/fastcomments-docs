FastComments のアクセス制御は、`Pages` と `Users` を目的のグループに割り当てることで機能します。

グループは単純に文字列識別子で、`GREEN` や `abc-123` のようなものです。

`Users` と `Pages` は 1 つのグループに限定されません。制限はそれぞれ `100` と `1000` のグループです。 

#### 許可されていないページへのアクセス

ユーザーがアクセス権のないページにアクセスしようとすると、以下のようなエラーメッセージが表示されます:

<div class="screenshot white-bg">
    <div class="title">認可失敗の例</div>
    <img class="screenshot-image" src="/images/sso-unauthorized-message.png" alt="認可失敗の例" />
</div>

メッセージテキストはカスタマイズできます。

---