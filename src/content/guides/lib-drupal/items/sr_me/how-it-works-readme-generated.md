Када корисник посети ентитет са омогућеним FastComments пољем:

1. FastComments JavaScript видџет се учитава са CDN-а.
2. Ако је SSO конфигурисан, Drupal идентитет корисника се прослеђује FastComments-у.
3. Надокнада `<noscript>` обезбеђује серверски рендероване коментаре за кориснике без JavaScript-а (само у режимима Live Comments и Streaming Chat).