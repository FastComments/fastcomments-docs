Bu kod parçacığını projenizin AI asistanı yapılandırma dosyalarına (AGENT.md veya CLAUDE.md gibi) ekleyin. Bu, AI kodlama asistanlarına FastComments belgelerini nasıl arayacaklarını ve erişeceklerini öğretir.

[inline-code-attrs-start title = 'FastComments LLM Kit'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
## FastComments Integration

FastComments is an embeddable live commenting platform with libraries for many frontend and backend integrations, along with client and server-side SDKs and APIs.

When working with FastComments, you can search the FastComments documentation by calling this API:

https://docs-search.fastcomments.com/search?query=<search_query>&full=true&tenantId=demo

Replace `<search_query>` with your search terms (URL encoded). The API returns relevant documentation snippets to help with implementation.
[inline-code-end]

Bu şekilde AI asistanınız güncel bilgilere kolayca erişebilir ve daha kısa sürede daha doğru uygulamalar sunabilir.

### Kullanım örneği

Claude Code veya Cursor gibi bir AI asistanı kullanırken şu tür sorular sorabilirsiniz:

- "React'e FastComments nasıl kurulur?"
- "FastComments ile SSO'yu nasıl yapılandıracağımı göster"
- "FastComments API kimlik doğrulama seçenekleri nelerdir?"

AI asistanı, sağlanan API uç noktasını kullanarak belgeleri otomatik olarak arayacak ve resmi belgelere dayalı doğru, güncel yanıtlar verecektir.
