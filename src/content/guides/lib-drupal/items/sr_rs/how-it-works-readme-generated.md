Када корисник посети ентитет са омогућеним FastComments пољем:

1. FastComments JavaScript виџет се учитава са CDN-а.
2. Ако је SSO конфигурисан, Drupal идентитет корисника се прослеђује FastComments-у.
3. Резервни `<noscript>` механизам обезбеђује коментаре рендероване на серверу за кориснике без JavaScript-а (само у режимима Live Comments и Streaming Chat).