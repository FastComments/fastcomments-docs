Disqus と FastComments Secure SSO の最大の違いは、Disqus が暗号化に SHA1 を使用しているのに対し、当社は SHA256 を使用している点です。

つまり、Disqus からの移行は簡単です - 使用するハッシュアルゴリズムを SHA1 から SHA256 に変更し、UI に渡されるプロパティ名を更新するだけです。