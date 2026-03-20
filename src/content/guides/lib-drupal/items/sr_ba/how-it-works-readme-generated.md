Када корисник посјети ентитет са омогућеним FastComments пољем:

1. FastComments JavaScript виџет се учитава са CDN-а.
2. Ако је SSO конфигурисан, Drupal идентитет корисника се преноси на FastComments.
3. Резервна опција `<noscript>` обезбјеђује коментаре рендероване на серверу за кориснике без JavaScript-а (само у режимима Live Comments и Streaming Chat).