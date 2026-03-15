Когато потребител посети entity, за която полето FastComments е активирано:

1. JavaScript widget-ът на FastComments се зарежда от CDN.
2. Ако SSO е конфигуриран, идентичността на потребителя в Drupal се предава на FastComments.
3. Резервният вариант <noscript> предоставя коментари, рендерирани от сървъра, за потребители без JavaScript (само в режимите Live Comments и Streaming Chat).