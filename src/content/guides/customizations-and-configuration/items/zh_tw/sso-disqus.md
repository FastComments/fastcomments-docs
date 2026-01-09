Disqus 與 FastComments Secure SSO 之間最大的差異是 Disqus 使用 SHA1 進行加密，而我們使用 SHA256。

這表示從 Disqus 遷移很容易 - 將使用的雜湊演算法從 SHA1 改為 SHA256，並更新傳遞給 UI 的屬性名稱。