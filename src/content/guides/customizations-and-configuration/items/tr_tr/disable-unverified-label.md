[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, FastComments, doğrulanmamış bir tarayıcı oturumuna sahip bir kullanıcı için bırakılan yorumlarda
"Unverified Comment" etiketini gösterir. Doğrulanmamış yorumlar hakkında daha fazla bilgiyi [buradan](https://docs.fastcomments.com/guide-comment-vote-verification.html) okuyun.

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Disable The Unverified Label'; code-example-end]

Additionally, this feature can be used, without writing code, in the Customization UI:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; title='Disable The Unverified Label' app-screenshot-end]

---