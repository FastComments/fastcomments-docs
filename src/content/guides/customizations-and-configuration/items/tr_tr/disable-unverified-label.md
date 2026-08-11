[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, FastComments, doğrulanmamış bir tarayıcı oturumuna sahip bir kullanıcı için bırakılan yorumlarda "Doğrulanmamış Yorum" etiketini gösterir. Doğrulanmamış yorumlama hakkında daha fazla bilgiyi [burada](https://docs.fastcomments.com/guide-comment-vote-verification.html) okuyabilirsiniz.

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Disable The Unverified Label'; code-example-end]

Ayrıca, bu özellik kod yazmadan, Özelleştirme UI'sinde kullanılabilir:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; alt='Widget özelleştirme sayfası, Disable Unverified Comment Label onay kutusu işaretli'; title='Doğrulanmamış Etiketi Devre Dışı Bırak' app-screenshot-end]