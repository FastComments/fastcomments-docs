When a user visits an entity with the FastComments field enabled:

1. The FastComments JavaScript widget is loaded from the CDN.
2. If SSO is configured, the user's Drupal identity is passed to FastComments.
3. A `<noscript>` fallback provides server-rendered comments for users without JavaScript (Live Comments and Streaming Chat modes only).